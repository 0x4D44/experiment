/**
 * Cyber Breach - Network Hacking Puzzle Game
 * A sophisticated hacking simulation with network nodes, puzzles, and stealth mechanics
 */

// ============================================================================
// TYPES AND INTERFACES
// ============================================================================

export enum NodeType {
  Gateway = 'gateway',
  Server = 'server',
  Database = 'database',
  SecurityHub = 'security_hub',
  DataCache = 'data_cache',
}

export enum PuzzleType {
  PasswordCrack = 'password_crack',
  PortScan = 'port_scan',
  Encryption = 'encryption',
  Privilege = 'privilege',
  Firewall = 'firewall',
}

export enum ToolType {
  PortScanner = 'port_scanner',
  PasswordCracker = 'password_cracker',
  FirewallBypass = 'firewall_bypass',
  PrivilegeExploit = 'privilege_exploit',
  NetworkMask = 'network_mask',
}

export interface Node {
  id: string;
  type: NodeType;
  x: number;
  y: number;
  securityLevel: number;
  puzzle: Puzzle;
  hacked: boolean;
  data: number;
  connections: string[];
}

export interface Puzzle {
  type: PuzzleType;
  difficulty: number;
  attempts: number;
  maxAttempts: number;
  solved: boolean;
  data?: string;
}

export interface Tool {
  type: ToolType;
  name: string;
  uses: number;
  active: boolean;
  effect: number;
}

export interface NetworkConnection {
  from: string;
  to: string;
  firewall: number;
  bandwidth: number;
}

export interface GameLevel {
  id: string;
  name: string;
  description: string;
  nodes: Node[];
  connections: NetworkConnection[];
  objective: string;
  timeLimit: number;
  detectionThreshold: number;
}

export interface GameState {
  currentLevel: number;
  nodes: Map<string, Node>;
  connections: NetworkConnection[];
  tools: Tool[];
  detection: number;
  score: number;
  timeRemaining: number;
  gameOver: boolean;
  levelComplete: boolean;
  activePath: string[];
}

// ============================================================================
// PUZZLE SYSTEM
// ============================================================================

export class PuzzleGenerator {
  static generatePasswordCrack(difficulty: number): Puzzle {
    const passwords = ['admin', '123456', 'password', 'letmein', 'welcome'];
    return {
      type: PuzzleType.PasswordCrack,
      difficulty,
      attempts: 0,
      maxAttempts: Math.max(3, 10 - difficulty),
      solved: false,
      data: passwords[Math.floor(Math.random() * passwords.length)],
    };
  }

  static generatePortScan(difficulty: number): Puzzle {
    const openPorts = [22, 80, 443, 3306, 5432, 8080, 8443];
    const selectedPorts = openPorts.slice(0, Math.ceil(difficulty / 2));
    return {
      type: PuzzleType.PortScan,
      difficulty,
      attempts: 0,
      maxAttempts: 5,
      solved: false,
      data: JSON.stringify({
        openPorts: selectedPorts,
        totalPorts: 65535,
        vulnerability: Math.random() > 0.5,
      }),
    };
  }

  static generateEncryption(difficulty: number): Puzzle {
    const shift = Math.floor(Math.random() * 25) + 1;
    const message = 'ENCRYPT_ME';
    const encrypted = message
      .split('')
      .map((c) => String.fromCharCode(((c.charCodeAt(0) - 65 + shift) % 26) + 65))
      .join('');

    return {
      type: PuzzleType.Encryption,
      difficulty,
      attempts: 0,
      maxAttempts: Math.max(2, 8 - difficulty),
      solved: false,
      data: JSON.stringify({ encrypted, shift }),
    };
  }

  static generatePrivilege(difficulty: number): Puzzle {
    return {
      type: PuzzleType.Privilege,
      difficulty,
      attempts: 0,
      maxAttempts: Math.max(2, 6 - difficulty),
      solved: false,
      data: JSON.stringify({
        required: ['execute', 'read', 'write'],
        vulnerable: difficulty > 5,
      }),
    };
  }

  static generateFirewall(difficulty: number): Puzzle {
    const rules = Math.ceil(difficulty / 2);
    return {
      type: PuzzleType.Firewall,
      difficulty,
      attempts: 0,
      maxAttempts: Math.max(3, 8 - difficulty),
      solved: false,
      data: JSON.stringify({ rules, protocols: ['tcp', 'udp', 'icmp'] }),
    };
  }

  static generatePuzzleForSecurity(nodeType: NodeType, difficulty: number): Puzzle {
    const puzzleTypes: { [key in NodeType]: PuzzleType } = {
      [NodeType.Gateway]: PuzzleType.PortScan,
      [NodeType.Server]: PuzzleType.PasswordCrack,
      [NodeType.Database]: PuzzleType.Encryption,
      [NodeType.SecurityHub]: PuzzleType.Privilege,
      [NodeType.DataCache]: PuzzleType.Firewall,
    };

    const type = puzzleTypes[nodeType];
    switch (type) {
      case PuzzleType.PasswordCrack:
        return this.generatePasswordCrack(difficulty);
      case PuzzleType.PortScan:
        return this.generatePortScan(difficulty);
      case PuzzleType.Encryption:
        return this.generateEncryption(difficulty);
      case PuzzleType.Privilege:
        return this.generatePrivilege(difficulty);
      case PuzzleType.Firewall:
        return this.generateFirewall(difficulty);
      default:
        return this.generatePasswordCrack(difficulty);
    }
  }
}

// ============================================================================
// NODE SYSTEM
// ============================================================================

export class NetworkNode {
  node: Node;

  constructor(
    id: string,
    type: NodeType,
    x: number,
    y: number,
    securityLevel: number,
  ) {
    this.node = {
      id,
      type,
      x,
      y,
      securityLevel: Math.min(10, Math.max(1, securityLevel)),
      puzzle: PuzzleGenerator.generatePuzzleForSecurity(type, securityLevel),
      hacked: type === NodeType.Gateway,
      data: securityLevel * 10,
      connections: [],
    };
  }

  canAccess(tools: Tool[]): boolean {
    if (this.node.hacked) return true;
    if (this.node.type === NodeType.Gateway) return true;

    const hasRelevantTool = tools.some(
      (t) =>
        t.active &&
        ((t.type === ToolType.PasswordCracker && this.node.type === NodeType.Server) ||
          (t.type === ToolType.FirewallBypass && this.node.type === NodeType.SecurityHub) ||
          (t.type === ToolType.PrivilegeExploit && this.node.type === NodeType.Database)),
    );

    return hasRelevantTool;
  }

  solvePuzzle(answer: string): boolean {
    if (this.node.puzzle.solved) return true;
    if (this.node.puzzle.attempts >= this.node.puzzle.maxAttempts) return false;

    this.node.puzzle.attempts++;

    switch (this.node.puzzle.type) {
      case PuzzleType.PasswordCrack:
        return answer === this.node.puzzle.data;

      case PuzzleType.PortScan: {
        const data = JSON.parse(this.node.puzzle.data || '{}');
        const ports = answer.split(',').map((p) => parseInt(p.trim()));
        return ports.some((p) => data.openPorts.includes(p));
      }

      case PuzzleType.Encryption: {
        return answer.toUpperCase() === 'ENCRYPT_ME';
      }

      case PuzzleType.Privilege: {
        const data = JSON.parse(this.node.puzzle.data || '{}');
        return data.required.every((r: string) => answer.includes(r));
      }

      case PuzzleType.Firewall: {
        return answer.includes('tcp') && answer.includes('udp');
      }

      default:
        return false;
    }
  }

  getSecurityInfo(): string {
    const puzzle = this.node.puzzle;
    let info = `Node: ${this.node.id}\n`;
    info += `Type: ${this.node.type}\n`;
    info += `Security: ${this.node.securityLevel}/10\n`;
    info += `Puzzle: ${puzzle.type}\n`;
    info += `Attempts: ${puzzle.attempts}/${puzzle.maxAttempts}\n`;

    if (puzzle.type === PuzzleType.PortScan && puzzle.data) {
      const data = JSON.parse(puzzle.data);
      info += `Open Ports: ${data.openPorts.length} found\n`;
    }

    return info;
  }
}

// ============================================================================
// TOOL SYSTEM
// ============================================================================

export class ToolKit {
  tools: Map<ToolType, Tool>;

  constructor() {
    this.tools = new Map([
      [
        ToolType.PortScanner,
        {
          type: ToolType.PortScanner,
          name: 'Port Scanner',
          uses: 3,
          active: false,
          effect: 1.5,
        },
      ],
      [
        ToolType.PasswordCracker,
        {
          type: ToolType.PasswordCracker,
          name: 'Password Cracker',
          uses: 2,
          active: false,
          effect: 2.0,
        },
      ],
      [
        ToolType.FirewallBypass,
        {
          type: ToolType.FirewallBypass,
          name: 'Firewall Bypass',
          uses: 1,
          active: false,
          effect: 2.5,
        },
      ],
      [
        ToolType.PrivilegeExploit,
        {
          type: ToolType.PrivilegeExploit,
          name: 'Privilege Exploit',
          uses: 2,
          active: false,
          effect: 1.8,
        },
      ],
      [
        ToolType.NetworkMask,
        {
          type: ToolType.NetworkMask,
          name: 'Network Mask',
          uses: -1,
          active: false,
          effect: 0.5,
        },
      ],
    ]);
  }

  activateTool(type: ToolType): boolean {
    const tool = this.tools.get(type);
    if (!tool) return false;
    if (tool.uses === 0) return false;

    tool.active = true;
    if (tool.uses > 0) tool.uses--;

    return true;
  }

  deactivateTool(type: ToolType): void {
    const tool = this.tools.get(type);
    if (tool) tool.active = false;
  }

  getTool(type: ToolType): Tool | undefined {
    return this.tools.get(type);
  }

  getActiveTools(): Tool[] {
    return Array.from(this.tools.values()).filter((t) => t.active);
  }

  getToolStatus(): string {
    let status = 'Tools:\n';
    this.tools.forEach((tool) => {
      const uses = tool.uses === -1 ? 'Unlimited' : tool.uses;
      status += `${tool.name}: ${uses} uses (${tool.active ? 'ACTIVE' : 'inactive'})\n`;
    });
    return status;
  }
}

// ============================================================================
// STEALTH & DETECTION SYSTEM
// ============================================================================

export class DetectionSystem {
  detection: number = 0;
  maxDetection: number = 100;
  scanInterval: number = 5;
  lastScanTime: number = 0;

  incrementDetection(amount: number, toolEffect: number = 1.0): void {
    const actualAmount = amount / toolEffect;
    this.detection = Math.min(this.maxDetection, this.detection + actualAmount);
  }

  decrementDetection(amount: number): void {
    this.detection = Math.max(0, this.detection - amount);
  }

  getDetectionLevel(): 'low' | 'medium' | 'high' | 'critical' {
    if (this.detection < 25) return 'low';
    if (this.detection < 50) return 'medium';
    if (this.detection < 75) return 'high';
    return 'critical';
  }

  isAlarmTriggered(): boolean {
    return this.detection >= this.maxDetection;
  }

  resetDetection(): void {
    this.detection = 0;
  }

  getDetectionInfo(): string {
    return `Detection: ${Math.round(this.detection)}/100 [${this.getDetectionLevel().toUpperCase()}]`;
  }
}

// ============================================================================
// LEVEL SYSTEM
// ============================================================================

export class LevelManager {
  static createLevel(levelNum: number): GameLevel {
    const baseSecurityLevel = Math.min(10, 2 + levelNum);
    const nodeCount = 3 + Math.min(7, levelNum);
    const nodes: Node[] = [];
    const connections: NetworkConnection[] = [];

    nodes.push(
      new NetworkNode(`gateway_${levelNum}`, NodeType.Gateway, 100, 100, 1).node,
    );

    const nodeTypes = [NodeType.Server, NodeType.Database, NodeType.SecurityHub, NodeType.DataCache];
    for (let i = 1; i < nodeCount; i++) {
      const type = nodeTypes[Math.floor(Math.random() * nodeTypes.length)];
      const x = 100 + Math.random() * 600;
      const y = 100 + Math.random() * 400;
      const security = Math.min(10, baseSecurityLevel + Math.floor(Math.random() * 3));

      nodes.push(new NetworkNode(`node_${levelNum}_${i}`, type, x, y, security).node);
    }

    for (let i = 0; i < nodeCount - 1; i++) {
      const from = nodes[i].id;
      const to = nodes[i + 1].id;
      const firewall = Math.floor(Math.random() * baseSecurityLevel) + 1;

      connections.push({
        from,
        to,
        firewall,
        bandwidth: 10 + Math.random() * 90,
      });

      if (Math.random() > 0.6) {
        const randomTarget = nodes[Math.floor(Math.random() * nodeCount)];
        if (randomTarget.id !== from) {
          connections.push({
            from,
            to: randomTarget.id,
            firewall: Math.floor(Math.random() * baseSecurityLevel) + 1,
            bandwidth: 10 + Math.random() * 90,
          });
        }
      }
    }

    const timeLimit = Math.max(60, 300 - levelNum * 20);

    return {
      id: `level_${levelNum}`,
      name: `Network ${levelNum}`,
      description: `Infiltrate the network and extract data. Security Level: ${baseSecurityLevel}`,
      nodes,
      connections,
      objective: `Extract ${Math.floor(nodeCount * 5)} data points`,
      timeLimit,
      detectionThreshold: 100 - levelNum * 5,
    };
  }

  static getAvailableLevels(): number {
    return 12;
  }
}

// ============================================================================
// GAME ENGINE
// ============================================================================

export class CyberBreachGame {
  gameState: GameState;
  detectionSystem: DetectionSystem;
  toolkit: ToolKit;
  currentLevel: GameLevel;
  startTime: number;

  constructor() {
    this.detectionSystem = new DetectionSystem();
    this.toolkit = new ToolKit();
    this.gameState = this.initializeGame();
    this.currentLevel = LevelManager.createLevel(1);
    this.startTime = Date.now();
  }

  private initializeGame(): GameState {
    return {
      currentLevel: 1,
      nodes: new Map(),
      connections: [],
      tools: Array.from(this.toolkit.tools.values()),
      detection: 0,
      score: 0,
      timeRemaining: 300,
      gameOver: false,
      levelComplete: false,
      activePath: [],
    };
  }

  loadLevel(levelNum: number): boolean {
    if (levelNum < 1 || levelNum > LevelManager.getAvailableLevels()) {
      return false;
    }

    this.currentLevel = LevelManager.createLevel(levelNum);
    this.gameState.currentLevel = levelNum;
    this.gameState.nodes.clear();
    this.gameState.connections = this.currentLevel.connections;
    this.gameState.activePath = [];
    this.detectionSystem.resetDetection();
    this.startTime = Date.now();

    this.currentLevel.nodes.forEach((node) => {
      this.gameState.nodes.set(node.id, node);
    });

    return true;
  }

  hackNode(nodeId: string, answer: string): { success: boolean; message: string } {
    const node = this.gameState.nodes.get(nodeId);
    if (!node) {
      return { success: false, message: 'Node not found' };
    }

    if (node.hacked) {
      return { success: true, message: 'Node already hacked' };
    }

    const networkNode = new NetworkNode(
      node.id,
      node.type,
      node.x,
      node.y,
      node.securityLevel,
    );
    networkNode.node = node;

    const canAccess = networkNode.canAccess(this.gameState.tools);
    if (!canAccess && node.type !== NodeType.Gateway) {
      return { success: false, message: 'Insufficient tools to access this node' };
    }

    const solved = networkNode.node.puzzle.solved || networkNode.solvePuzzle(answer);

    if (solved) {
      node.hacked = true;
      node.puzzle.solved = true;
      const dataPoints = node.data;
      this.gameState.score += dataPoints;
      this.gameState.activePath.push(nodeId);

      const toolMultiplier = this.toolkit.getTool(ToolType.NetworkMask)?.active ? 0.5 : 1.0;
      this.detectionSystem.incrementDetection(node.securityLevel * 3, 1 / toolMultiplier);

      return {
        success: true,
        message: `Success! Extracted ${dataPoints} data points. Detection: ${Math.round(this.detectionSystem.detection)}`,
      };
    } else {
      this.detectionSystem.incrementDetection(node.securityLevel * 2);

      if (node.puzzle.attempts >= node.puzzle.maxAttempts) {
        return {
          success: false,
          message: `Failed! Puzzle locked (${node.puzzle.attempts}/${node.puzzle.maxAttempts} attempts). Detection: ${Math.round(this.detectionSystem.detection)}`,
        };
      }

      return {
        success: false,
        message: `Incorrect! Detection: ${Math.round(this.detectionSystem.detection)} (${node.puzzle.attempts}/${node.puzzle.maxAttempts})`,
      };
    }
  }

  update(deltaTime: number): void {
    const elapsed = (Date.now() - this.startTime) / 1000;
    this.gameState.timeRemaining = Math.max(
      0,
      this.currentLevel.timeLimit - elapsed,
    );

    if (this.gameState.timeRemaining === 0) {
      this.gameState.gameOver = true;
    }

    if (this.detectionSystem.isAlarmTriggered()) {
      this.gameState.gameOver = true;
    }

    const targetDataPoints = this.currentLevel.nodes.reduce((sum, n) => sum + n.data, 0) / 2;
    if (this.gameState.score >= targetDataPoints) {
      this.gameState.levelComplete = true;
    }
  }

  getGameStatus(): string {
    let status = '\n=== CYBER BREACH ===\n';
    status += `Level: ${this.gameState.currentLevel}\n`;
    status += `Time: ${Math.ceil(this.gameState.timeRemaining)}s\n`;
    status += `Score: ${this.gameState.score}\n`;
    status += this.detectionSystem.getDetectionInfo() + '\n';
    status += '\nNetwork Nodes:\n';

    this.gameState.nodes.forEach((node) => {
      const nodeStatus = node.hacked ? '[HACKED]' : '[SECURE]';
      status += `${nodeStatus} ${node.id} (${node.type})\n`;
    });

    return status;
  }

  listNodes(): string[] {
    return Array.from(this.gameState.nodes.keys());
  }

  getNodeInfo(nodeId: string): string {
    const node = this.gameState.nodes.get(nodeId);
    if (!node) return 'Node not found';

    const networkNode = new NetworkNode(
      node.id,
      node.type,
      node.x,
      node.y,
      node.securityLevel,
    );
    networkNode.node = node;

    return networkNode.getSecurityInfo();
  }

  getConnectionsInfo(): string {
    let info = 'Network Connections:\n';
    this.gameState.connections.forEach((conn) => {
      info += `${conn.from} -> ${conn.to} (Firewall: ${conn.firewall}, Bandwidth: ${Math.round(conn.bandwidth)}%)\n`;
    });
    return info;
  }

  getToolStatus(): string {
    return this.toolkit.getToolStatus();
  }

  activateTool(toolType: string): boolean {
    const toolTypeKey = Object.entries(ToolType).find(
      ([_, v]) => v === toolType,
    )?.[1];
    if (!toolTypeKey) return false;

    return this.toolkit.activateTool(toolTypeKey as ToolType);
  }

  deactivateTool(toolType: string): void {
    const toolTypeKey = Object.entries(ToolType).find(
      ([_, v]) => v === toolType,
    )?.[1];
    if (toolTypeKey) {
      this.toolkit.deactivateTool(toolTypeKey as ToolType);
    }
  }

  nextLevel(): boolean {
    if (this.gameState.currentLevel < LevelManager.getAvailableLevels()) {
      return this.loadLevel(this.gameState.currentLevel + 1);
    }
    return false;
  }
}

export default CyberBreachGame;
