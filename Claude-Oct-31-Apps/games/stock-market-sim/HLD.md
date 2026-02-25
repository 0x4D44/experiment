# Stock Market Simulator - High Level Design (HLD)

## 1. Market Simulation Architecture

### 1.1 System Components
```
┌─────────────────────────────────────────────────────────────┐
│                    MARKET SIMULATOR                         │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Market Core  │  │ Price Engine │  │ Order Engine │      │
│  │              │  │              │  │              │      │
│  │ - Calendar   │  │ - GBM Model  │  │ - Matching   │      │
│  │ - Hours      │  │ - Volatility │  │ - Settlement │      │
│  │ - Events     │  │ - Correlation │ │ - Slippage   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Portfolio    │  │ Analytics    │  │ Persistence  │      │
│  │ Management   │  │ Engine       │  │ Layer        │      │
│  │              │  │              │  │              │      │
│  │ - Holdings   │  │ - Indicators │  │ - Save/Load  │      │
│  │ - Cash       │  │ - Greeks     │  │ - Snapshots  │      │
│  │ - Margin     │  │ - Performance│  │ - History    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ AI Traders   │  │ UI/Display   │  │ News & Events│      │
│  │              │  │              │  │              │      │
│  │ - 5 Types    │  │ - Dashboard  │  │ - News Feed │      │
│  │ - Strategies │  │ - Charts     │  │ - Indicators │      │
│  │ - Learning   │  │ - Portfolio  │  │ - Random     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 Concurrency Model
- **Market Update Loop**: Goroutine-based real-time price updates
- **Event Processing**: Channel-based async event handling
- **UI Rendering**: Non-blocking display updates
- **Data Synchronization**: Mutex-protected shared state

## 2. Price Movement Algorithms

### 2.1 Geometric Brownian Motion (GBM)
```
dS/S = μdt + σdW

Where:
- S = Stock price
- μ = Drift (mean return)
- σ = Volatility
- dW = Wiener process increment
- dt = Time step
```

**Implementation**:
```
S(t+dt) = S(t) * exp((μ - σ²/2) * dt + σ * sqrt(dt) * Z)
```
Where Z ~ N(0,1)

### 2.2 Volatility Clustering (GARCH-like)
- Current volatility influences future volatility
- Implements mean-reversion to long-term vol
- Sector correlation increases volatility spread
- Random shock events spike volatility temporarily

### 2.3 Mean Reversion
- Stocks drift toward intrinsic value
- Reversion strength varies by stock type
- Creates trading opportunities
- Prevents explosive price divergence

### 2.4 Momentum & Trending
- 10-20 day momentum carried forward
- Probability of trend continuation: 55-65%
- Trend fatigue after 60+ days
- Reversal probability increases with magnitude

### 2.5 Market Correlation
- Sector-level correlations: 0.3-0.7
- Market-level correlations: 0.4-0.6
- Correlation matrix updated daily
- Contagion effects for major events

### 2.6 Intraday Patterns
- U-shaped volume profile (high open/close)
- Volatility variations through day
- Bid-ask spread dynamics
- Trading halt simulation

## 3. Portfolio Management System

### 3.1 Account Types
```
┌─────────────────────────────────┐
│     PORTFOLIO MANAGEMENT        │
├─────────────────────────────────┤
│                                  │
│  CASH ACCOUNT                    │
│  ├─ No leverage                 │
│  ├─ No short selling             │
│  └─ Simple P/L calculation       │
│                                  │
│  MARGIN ACCOUNT                  │
│  ├─ 2:1 leverage (basic)        │
│  ├─ Margin interest: 7% annual  │
│  ├─ Maintenance requirement: 30% │
│  └─ Margin calls when <30%      │
│                                  │
│  SHORT SELLING                   │
│  ├─ Borrow stock (1% daily fee)  │
│  ├─ Unlimited losses             │
│  ├─ Dividends payable to lender  │
│  └─ Buy to cover                 │
│                                  │
└─────────────────────────────────┘
```

### 3.2 Portfolio Structure
```
struct Portfolio {
    Cash float64                      // Available cash
    Holdings map[string]*Position    // Stock positions
    Orders []Order                   // Open orders
    TradeHistory []Trade             // Closed trades
    Dividends float64                // Dividend payout
    MarginUsed float64               // Current margin used
    BuyingPower float64              // Margin * 2
}

struct Position {
    Symbol string
    Shares float64
    CostBasis float64                // Average cost per share
    EntryPrice float64
    CurrentPrice float64
    Realized P/L
    Unrealized P/L
    DayP/L float64
}
```

### 3.3 Risk Management
- **Portfolio Value Calculation**:
  ```
  NetValue = Cash + (Holdings * Current Prices) - Margin Interest - Loans
  ```

- **Margin Requirements**:
  ```
  MarginRatio = (NetValue - Loans) / (Current Holdings Value)
  Minimum Ratio = 30% (maintenance)
  ```

- **Dividends**:
  - Quarterly distributions
  - Reinvestment options
  - Ex-dividend date tracking
  - Record date handling

## 4. Order Types and Execution Engine

### 4.1 Supported Order Types
```
┌─────────────────────────────────────────────┐
│           ORDER TYPES & EXECUTION           │
├─────────────────────────────────────────────┤
│                                              │
│  MARKET ORDER                               │
│  ├─ Execute immediately at best price      │
│  ├─ Slippage: 0.01-0.1% based on volume    │
│  └─ Priority: Price > Size                  │
│                                              │
│  LIMIT ORDER                                │
│  ├─ Buy limit: Execute at/below limit      │
│  ├─ Sell limit: Execute at/above limit     │
│  ├─ Time-in-Force: Day, GTC                │
│  └─ Unfilled after day: Cancels            │
│                                              │
│  STOP LOSS                                  │
│  ├─ Sell when price drops below trigger    │
│  ├─ Becomes market order when triggered    │
│  └─ Risk management tool                    │
│                                              │
│  STOP LIMIT                                 │
│  ├─ Combines stop + limit orders           │
│  ├─ Execute at limit once stop triggered   │
│  └─ Risk: May never execute                 │
│                                              │
│  TRAILING STOP                              │
│  ├─ Stop trigger trails price upward       │
│  ├─ Locked in gains                         │
│  ├─ Dollar or percentage based              │
│  └─ Perfect for momentum trades             │
│                                              │
│  BRACKET ORDER                              │
│  ├─ Simultaneous entry + profit target     │
│  ├─ Profit target + stop loss               │
│  └─ OCO (One-Cancels-Other)                 │
│                                              │
└─────────────────────────────────────────────┘
```

### 4.2 Order Execution Engine
```
┌─────────────────────────────┐
│   ORDER PROCESSING FLOW     │
├─────────────────────────────┤
│                              │
│  1. VALIDATION              │
│     ├─ Syntax check         │
│     ├─ Account status       │
│     ├─ Buying power         │
│     └─ Margin check         │
│                              │
│  2. MATCHING                │
│     ├─ Order book lookup    │
│     ├─ Price matching       │
│     ├─ Quantity matching    │
│     └─ Partial fill support │
│                              │
│  3. SETTLEMENT              │
│     ├─ T+0 for simulation   │
│     ├─ Cash deduction       │
│     ├─ Position update      │
│     └─ Commission deduction │
│                              │
│  4. REPORTING               │
│     ├─ Trade confirmation   │
│     ├─ Order history        │
│     ├─ P/L calculation      │
│     └─ Tax report data      │
│                              │
└─────────────────────────────┘
```

### 4.3 Costs & Fees
```
Market Order:
├─ Commission: $0-10 per trade (varies by mode)
├─ Spread Cost: (ask - bid) / mid * 100
└─ Slippage: 0.01-0.1% based on liquidity

Short Selling:
├─ Borrow Fee: 1% annually
├─ Dividend Payback: To share lender
└─ Buyback Commission: Standard commissions

Margin Account:
├─ Interest Rate: 7% annually
├─ Calculated on: Sum of short positions
└─ Charged: Daily accrual
```

## 5. Market Events and News System

### 5.1 Event Categories
```
┌────────────────────────────────────────────┐
│           EVENT TAXONOMY                   │
├────────────────────────────────────────────┤
│                                             │
│  COMPANY-SPECIFIC (Individual Stocks)      │
│  ├─ Earnings Beat: +1-3%                  │
│  ├─ Earnings Miss: -1-3%                  │
│  ├─ Product Launch: +0.5-2%               │
│  ├─ Recall/Crisis: -2-5%                  │
│  ├─ FDA Approval: +2-8% (pharma)          │
│  ├─ Regulatory Action: -1-4%              │
│  ├─ CEO Change: -1-3%                     │
│  ├─ Acquisition: +2-10%                   │
│  └─ Spinoff: -1-2%                        │
│                                             │
│  SECTOR EVENTS (Groups of Stocks)          │
│  ├─ Energy Crisis: Energy +5-15%          │
│  ├─ Rate Hike: Tech -3-5%, Banks +2-4%   │
│  ├─ Oil Collapse: Energy -10-20%          │
│  ├─ Tech Bubble: Tech +10-20%             │
│  └─ Sector Rotation: Cross-sector impact  │
│                                             │
│  MARKET-WIDE (All Stocks)                  │
│  ├─ Bull Market: All +0.5-2%/day          │
│  ├─ Bear Market: All -0.5-2%/day          │
│  ├─ Black Swan: -5-20% single day         │
│  ├─ Circuit Breaker: Trading halt         │
│  ├─ Fed Announcement: +/-2-5%             │
│  └─ Geopolitical: -2-10%                  │
│                                             │
│  SCHEDULED EVENTS                          │
│  ├─ Earnings Calendar: Known in advance   │
│  ├─ Economic Data: CPI, Jobs, GDP         │
│  ├─ Fed Meetings: Scheduled              │
│  └─ Index Rebalancing: Quarterly         │
│                                             │
└────────────────────────────────────────────┘
```

### 5.2 Impact Modeling
```
Price Impact = Base_Impact * Duration_Decay * Contagion_Multiplier

Base Impact Calculation:
├─ News Sentiment: -3 to +3 scale
├─ Surprise Factor: 0.5x to 2.0x
├─ Stock Volatility: Scales impact
└─ Market Conditions: Bull/bear adjustment

Duration (Sentiment Fade):
├─ Day 1: 100% intensity
├─ Day 2-3: 75% intensity
├─ Day 4-7: 50% intensity
└─ Day 8+: 25% intensity

Contagion:
├─ Same sector: +0.5x impact
├─ Related sector: +0.2x impact
├─ Index components: +0.1x impact
└─ Market-wide events: All stocks affected
```

### 5.3 News Feed Integration
```
struct NewsEvent {
    ID string
    Timestamp time.Time
    Type EventType
    Headline string
    Description string
    Stocks []string                // Affected symbols
    ImpactScore float64            // -3 to +3
    Duration int                   // Days
    SentimentFade []float64        // Daily decay
}
```

## 6. Technical Indicators Implementation

### 6.1 Trend Indicators
```
SIMPLE MOVING AVERAGE (SMA)
├─ Formula: SMA(n) = Sum(Close[n]) / n
├─ Periods: 20, 50, 200 (common)
├─ Use: Trend identification, support/resistance
└─ Lag: High (lagging indicator)

EXPONENTIAL MOVING AVERAGE (EMA)
├─ Formula: EMA = (Close * Alpha) + (Previous EMA * (1 - Alpha))
├─ Alpha = 2 / (n + 1)
├─ Periods: 12, 26 (MACD inputs)
├─ Use: Faster trend following
└─ Lag: Lower than SMA

MOVING AVERAGE CONVERGENCE DIVERGENCE (MACD)
├─ Line 1 = EMA(12) - EMA(26)
├─ Signal = EMA(9) of MACD Line
├─ Histogram = MACD - Signal
├─ Signals: Crossovers, divergence
└─ Use: Momentum, trend changes
```

### 6.2 Momentum Indicators
```
RELATIVE STRENGTH INDEX (RSI)
├─ Formula: RSI = 100 - (100 / (1 + RS))
├─ Where RS = Average Gain / Average Loss
├─ Period: 14 (standard)
├─ Range: 0-100
├─ Signals:
│  ├─ >70: Overbought
│  ├─ <30: Oversold
│  └─ Divergence: Potential reversals
└─ Use: Entry/exit signals, confirmation

STOCHASTIC OSCILLATOR
├─ %K = (Close - Low14) / (High14 - Low14) * 100
├─ %D = SMA(3) of %K
├─ Signals: Oversold (<20), Overbought (>80)
├─ Crossovers: %K crosses %D
└─ Use: Momentum confirmation
```

### 6.3 Volatility Indicators
```
BOLLINGER BANDS
├─ Middle: SMA(20)
├─ Upper: SMA(20) + (2 * StdDev)
├─ Lower: SMA(20) - (2 * StdDev)
├─ Signal: Price outside bands = Extreme
├─ Width: Band width = volatility measure
└─ Use: Breakout detection, reversals

ATR (Average True Range)
├─ TR = max(H-L, |H-Close|, |L-Close|)
├─ ATR = EMA(14) of TR
├─ Use: Stop placement, position sizing
└─ Scaling: ATR % of price = volatility %

HISTORICAL VOLATILITY
├─ Formula: StdDev of log returns
├─ Period: 20, 30, 60 day
├─ Annualization: sqrt(252) * daily
└─ Use: Risk assessment, option pricing
```

### 6.4 Volume Indicators
```
ON-BALANCE VOLUME (OBV)
├─ Rules:
│  ├─ If Close > Prev Close: Add Volume
│  ├─ If Close < Prev Close: Subtract Volume
│  └─ If Close = Prev Close: No change
├─ Use: Trend confirmation
└─ Signal: Divergence from price

VOLUME RATE OF CHANGE (VROC)
├─ Formula: (Volume - Volume[n]) / Volume[n]
├─ Period: 12, 14
├─ Use: Volume trend, confirmation
└─ High VROC: Increased participation

VOLUME WEIGHTED AVERAGE PRICE (VWAP)
├─ Formula: Sum(Close * Volume) / Sum(Volume)
├─ Use: Institutional execution level
├─ Daytrading anchor point
└─ Mean reversion target
```

### 6.5 Calculation Storage
```
struct TechnicalIndicators {
    // Moving Averages
    SMA20, SMA50, SMA200 float64
    EMA12, EMA26 float64

    // MACD
    MACD float64
    MACDSignal float64
    MACDHistogram float64

    // Momentum
    RSI14 float64
    StochasticK float64
    StochasticD float64

    // Volatility
    BBUpper, BBMiddle, BBLower float64
    ATR14 float64
    HistoricalVol30 float64

    // Volume
    OBV float64
    VROC12 float64
    VWAP float64

    // Price Action
    Support, Resistance float64
    Trend TrendDirection
    Momentum float64              // -100 to +100
}
```

## 7. AI Trader Opponents

### 7.1 Trader Types
```
┌─────────────────────────────────────────────┐
│          AI TRADER PERSONALITIES            │
├─────────────────────────────────────────────┤
│                                              │
│  DAY TRADER (Aggressive, Fast)              │
│  Strategy:                                  │
│  ├─ Hold periods: Minutes to hours         │
│  ├─ Entry: Moving avg crossovers            │
│  ├─ Exit: 1-3% profit target or stops      │
│  ├─ Focus: Technical analysis              │
│  ├─ Risk: High turnover, commission impact │
│  ├─ Sample Symbols: High-volume tech       │
│  └─ Win Rate Target: 55-60%                │
│                                              │
│  VALUE INVESTOR (Patient, Fundamental)      │
│  Strategy:                                  │
│  ├─ Hold periods: Months to years          │
│  ├─ Entry: P/E < market, high dividend    │
│  ├─ Exit: Target price reached or thesis breaks │
│  ├─ Focus: P/E, dividend yield, growth    │
│  ├─ Research: Analyzes earnings           │
│  ├─ Sample Symbols: Large-cap blue chips  │
│  └─ Win Rate Target: 60-70%               │
│                                              │
│  MOMENTUM TRADER (Trend Follower)           │
│  Strategy:                                  │
│  ├─ Hold periods: Days to weeks            │
│  ├─ Entry: New 52-week highs, strong RSI  │
│  ├─ Exit: Momentum divergence, stops      │
│  ├─ Focus: Trend strength & continuation  │
│  ├─ Risk: Catches falling knives           │
│  ├─ Sample Symbols: Growth, trending      │
│  └─ Win Rate Target: 50-55%               │
│                                              │
│  CONTRARIAN (Counter-Trend)                │
│  Strategy:                                  │
│  ├─ Hold periods: 2-4 weeks                │
│  ├─ Entry: Oversold, down 20%+ from highs │
│  ├─ Exit: Mean reversion, 10-20% gains    │
│  ├─ Focus: Sentiment, technical extremes  │
│  ├─ Risk: Can be early, catches falling knives │
│  ├─ Sample Symbols: Beaten-down stocks    │
│  └─ Win Rate Target: 55-65%               │
│                                              │
│  INDEX FUND (Passive, Systematic)          │
│  Strategy:                                  │
│  ├─ Hold periods: Long-term (years)       │
│  ├─ Entry: Dollar-cost averaging          │
│  ├─ Exit: Buy and hold, rebalancing       │
│  ├─ Focus: Diversification, low cost      │
│  ├─ Risk: Beta matching, no alpha         │
│  ├─ Sample Symbols: Index components      │
│  └─ Win Rate Target: Matches market       │
│                                              │
└─────────────────────────────────────────────┘
```

### 7.2 Decision Making Algorithm
```
┌──────────────────────────────────────────┐
│      AI DECISION MAKING FLOW             │
├──────────────────────────────────────────┤
│                                           │
│  1. MARKET ANALYSIS                      │
│     ├─ Update indicators for all stocks │
│     ├─ Calculate scores by strategy     │
│     ├─ Identify entry/exit signals      │
│     └─ Rank candidates                  │
│                                           │
│  2. POSITION MANAGEMENT                  │
│     ├─ Review existing positions        │
│     ├─ Calculate unrealized P/L        │
│     ├─ Check stop loss / profit targets │
│     └─ Close winners/losers             │
│                                           │
│  3. RISK ASSESSMENT                      │
│     ├─ Portfolio concentration          │
│     ├─ Sector exposure                  │
│     ├─ Beta-adjusted exposure           │
│     └─ Margin availability              │
│                                           │
│  4. ORDER PLACEMENT                      │
│     ├─ Size: Based on risk rules        │
│     ├─ Price: Limit or market          │
│     ├─ Type: Market, stop, limit       │
│     └─ Execution: Immediate or wait    │
│                                           │
│  5. LEARNING                            │
│     ├─ Win/loss tracking                │
│     ├─ Strategy adjustment              │
│     ├─ Parameter tuning                 │
│     └─ Model retraining (quarterly)     │
│                                           │
└──────────────────────────────────────────┘
```

### 7.3 Learning Mechanism
```
Performance Tracking:
├─ Win Rate: Wins / Total Trades
├─ Avg Win: Average profit on winners
├─ Avg Loss: Average loss on losers
├─ Profit Factor: Gross Profit / Gross Loss
├─ Sharpe Ratio: Risk-adjusted returns
└─ Max Drawdown: Largest peak-to-trough decline

Adaptation:
├─ Weekly review of metrics
├─ Parameter adjustment (±5-10%)
├─ Strategy switching if P&L negative 2+ weeks
├─ Conservative mode if volatility spikes
└─ Aggressive mode if win rate >65%
```

## 8. Data Persistence and Save System

### 8.1 Save Structure
```
games/stock-market-sim/
├── data/
│   ├── saves/
│   │   ├── game_1.save.json         // Compressed game state
│   │   ├── game_2.save.json
│   │   └── saves.index              // Index of all saves
│   ├── stocks/
│   │   ├── stocks.csv               // 50+ stock definitions
│   │   └── sectors.csv              // Sector classifications
│   ├── history/
│   │   ├── prices_YYYY_MM_DD.json   // Price history
│   │   ├── trades_history.json      // All trades
│   │   └── events_history.json      // News events
│   └── snapshots/
│       └── snapshot_YYYY_MM_DD.json // Daily snapshots
```

### 8.2 Save File Format
```json
{
  "version": "1.0",
  "gameID": "uuid",
  "createdAt": "2025-10-31T14:30:00Z",
  "gameTime": {
    "simulationDay": 247,
    "currentTime": "2025-10-31T14:32:00Z",
    "marketOpen": true,
    "speed": 1.0
  },
  "portfolio": {
    "cash": 12451.23,
    "totalValue": 47832.15,
    "holdings": {
      "MEGA": {
        "shares": 100,
        "costBasis": 13000,
        "unrealizedPL": 1235
      }
    },
    "orders": [
      {
        "orderID": "uuid",
        "symbol": "TECH",
        "type": "LIMIT",
        "side": "BUY",
        "quantity": 250,
        "limitPrice": 45.50,
        "filledQuantity": 0,
        "createdAt": "2025-10-31T14:00:00Z"
      }
    ]
  },
  "marketState": {
    "stocks": {
      "MEGA": {
        "currentPrice": 142.35,
        "previousClose": 139.20,
        "high": 143.50,
        "low": 141.20,
        "volume": 3250000
      }
    },
    "indices": {
      "SNP500": 4247.83,
      "TECHDAQ": 13892.21
    }
  },
  "tradeHistory": [...],
  "performance": {
    "totalReturn": 0.183,
    "dayPL": 1247.82,
    "totalPL": 7832.15,
    "winRate": 0.62,
    "sharpeRatio": 1.45
  }
}
```

### 8.3 Compression & Performance
- Gzip compression for large history files
- Delta encoding for price histories
- Lazy loading of trade history
- Memory-efficient OHLCV bars
- Automatic cleanup of old snapshots (>90 days)

## 9. Performance Optimization for Real-Time Updates

### 9.1 Architecture Optimizations
```
CONCURRENCY MODEL:
├─ Market Update Goroutine (50ms tick)
│  ├─ Price generation (parallelized)
│  ├─ Technical indicator updates
│  ├─ Event processing
│  └─ Order matching
│
├─ UI Render Goroutine (100ms tick)
│  ├─ Display refresh
│  ├─ Portfolio updates
│  ├─ Chart rendering
│  └─ Non-blocking I/O
│
├─ AI Decision Goroutines (1s tick)
│  ├─ 5 concurrent AI traders
│  ├─ Strategy evaluation
│  ├─ Order placement
│  └─ Learning updates
│
└─ Persistence Goroutine (60s tick)
   ├─ Auto-save game state
   ├─ History snapshots
   └─ Database updates
```

### 9.2 Data Structure Optimization
```
Price Ring Buffer:
├─ Fixed-size circular buffer
├─ 252 trading days * 390 minutes = 98,280 entries
├─ Per stock: 64 bytes per entry (price, volume, etc)
├─ Total: ~50 stocks * 98K * 64 = 312 MB
├─ In-memory for speed
└─ Compressed on disk

Indicator Caching:
├─ Recalculate only when needed
├─ Cache TTL: 1 minute
├─ Batch updates per 5 stocks
└─ Lock-free reads where possible

Order Book:
├─ Hash map for O(1) lookup
├─ Sorted slice for matching
├─ Memory pool for allocations
└─ Batch processing mode
```

### 9.3 Calculation Efficiency
```
Price Updates (per tick):
├─ 50 stocks * GBM calculation = 50 μs
├─ Parallel processing = 10 μs
├─ Volume generation = 5 μs
├─ OHLC aggregation = 10 μs
└─ Total: ~25 μs per 50ms tick (0.05% CPU)

Indicator Updates (per minute):
├─ SMA/EMA: 20 ops per stock
├─ MACD/RSI: 100 ops per stock
├─ Bollinger Bands: 50 ops per stock
├─ Parallel batch: ~5 ms per update
└─ 50 stocks: ~5 ms total

Order Matching (per tick):
├─ Check open orders: O(log n)
├─ Match logic: O(1) expected
├─ Settlement: O(1)
└─ Typical: <1 ms per tick
```

### 9.4 Memory Management
```
Allocation Strategy:
├─ Pre-allocate arrays on startup
├─ Object pooling for frequently allocated items
├─ Ring buffers for historical data
├─ Lazy loading for trade history
└─ Compression for saved games

Memory Budget:
├─ Core data: ~100 MB
├─ Price history (252 days): ~150 MB
├─ Trade history (1000s trades): ~50 MB
├─ UI buffers: ~10 MB
└─ Total: ~300 MB target
```

## 10. File Organization & Dependencies

### 10.1 Package Structure
```
games/stock-market-sim/
├── main.go                    // Entry point
├── go.mod                     // Module definition
├── go.sum                     // Dependency checksums
├── Makefile                   // Build automation
├── README.md                  // Documentation
├── HLD.md                     // This document
│
├── cmd/
│   └── simulator/
│       └── main.go            // CLI entry
│
├── internal/
│   ├── market/
│   │   ├── market.go          // Market core
│   │   ├── price.go           // Price generation
│   │   ├── event.go           // Event system
│   │   └── calendar.go        // Trading hours
│   │
│   ├── portfolio/
│   │   ├── portfolio.go       // Portfolio management
│   │   ├── order.go           // Order types
│   │   ├── execution.go       // Order execution
│   │   └── position.go        // Position tracking
│   │
│   ├── analytics/
│   │   ├── indicators.go      // Technical indicators
│   │   ├── calculator.go      // Calculations
│   │   ├── performance.go     // P&L, Sharpe, etc
│   │   └── charting.go        // ASCII charts
│   │
│   ├── ai/
│   │   ├── trader.go          // Trader interface
│   │   ├── daytrader.go       // Day trader impl
│   │   ├── valueinvestor.go   // Value investor
│   │   ├── momentum.go        // Momentum trader
│   │   ├── contrarian.go      // Contrarian trader
│   │   └── indexfund.go       // Index fund trader
│   │
│   ├── ui/
│   │   ├── display.go         // Terminal display
│   │   ├── dashboard.go       // Main dashboard
│   │   ├── portfolio.go       // Portfolio view
│   │   ├── chart.go           // Chart rendering
│   │   └── input.go           // User input
│   │
│   ├── storage/
│   │   ├── save.go            // Save/load game
│   │   ├── persistence.go     // Data persistence
│   │   ├── history.go         // History tracking
│   │   └── compress.go        // Compression utils
│   │
│   └── utils/
│       ├── math.go            // Math utilities
│       ├── formatting.go      // Display formatting
│       ├── random.go          // RNG utilities
│       └── logger.go          // Logging
│
├── data/
│   ├── stocks.csv             // Stock definitions
│   ├── sectors.csv            // Sector data
│   ├── news_templates.json    // News event templates
│   └── saves/                 // Game saves
│
└── tests/
    ├── market_test.go         // Market tests
    ├── portfolio_test.go      // Portfolio tests
    ├── indicators_test.go     // Indicator tests
    ├── ai_test.go             // AI trader tests
    ├── execution_test.go      // Order execution tests
    └── integration_test.go    // End-to-end tests
```

### 10.2 External Dependencies
```
Standard Library (Primary):
├─ math, math/rand           // Calculations, randomness
├─ time                       // Time operations
├─ encoding/json              // JSON marshaling
├─ sync                       // Concurrency primitives
├─ fmt, io                    // I/O and formatting
├─ flag                       // CLI flags
└─ os, path/filepath          // File operations

Third-party (Minimal):
├─ github.com/mattn/go-runewidth  // Terminal width
├─ github.com/rivo/uniseg         // Unicode handling
└─ compress/gzip                  // Compression

Optional:
├─ github.com/jmoiron/sqlc       // For database (future)
└─ testing/quick, testing         // Testing utilities
```

## 11. Key Design Decisions & Rationale

### 11.1 Why Geometric Brownian Motion?
- Mathematically proven stock behavior model
- Produces realistic price distributions
- Supports volatility clustering
- Parameterizable for different stock types
- Industry standard for finance

### 11.2 Why Concurrent Architecture?
- Real-time feeling requires fast updates
- Separates concerns (market, UI, AI)
- Scales to hundreds of stocks
- Non-blocking UI for responsiveness
- Easier to test components in isolation

### 11.3 Why Multiple AI Traders?
- Educational: See different strategies in action
- Competitive: Beat other AI traders
- Realistic: Markets have various player types
- Research: Study strategy effectiveness
- Engagement: Dynamic opponent behavior

### 11.4 Why No Real Market Data?
- Simplification for simulation focus
- Consistent reproducible games
- Fast startup (no data download)
- Educational clarity
- Can add historical data later

## 12. Testing Strategy

### 12.1 Test Coverage Goals
- Market price generation: 95% coverage
- Portfolio calculations: 100% coverage
- Order execution: 100% coverage
- Technical indicators: 90% coverage
- AI decision making: 80% coverage
- Overall target: >85% coverage

### 12.2 Test Categories
1. Unit tests: Individual functions/methods
2. Integration tests: Component interactions
3. Benchmark tests: Performance validation
4. Simulation tests: Multi-day scenarios
5. Regression tests: Known bugs

## 13. Future Enhancements

### 13.1 Phase 2 Features
- Options trading (calls, puts, greeks)
- Cryptocurrency pairs
- Futures trading
- Real market data integration
- Machine learning trader
- Multiplayer mode

### 13.2 Phase 3 Features
- Mobile app
- Browser-based UI
- Advanced charting (Plotly)
- Social trading features
- Leaderboards
- Replay system

---

## Summary

This high-level design provides a comprehensive architecture for a realistic Stock Market Simulator with:
- Mathematically sound price generation
- Complete trading functionality
- Sophisticated AI opponents
- Rich analysis tools
- Educational value
- Production-ready code quality
- Excellent test coverage

The modular design allows for incremental implementation and testing, while the concurrent architecture ensures real-time responsiveness for an engaging gaming experience.
