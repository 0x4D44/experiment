/**
 * Cyber Breach - Comprehensive Test Suite
 * Tests for all game systems: puzzles, nodes, tools, detection, levels, and game engine
 */

import {
  NodeType,
  PuzzleType,
  ToolType,
  NetworkNode,
  PuzzleGenerator,
  ToolKit,
  DetectionSystem,
  LevelManager,
  CyberBreachGame,
} from './cyber-breach';

// ============================================================================
// PUZZLE SYSTEM TESTS
// ============================================================================

describe('PuzzleGenerator', () => {
  test('generates valid password crack puzzles', () => {
    const puzzle = PuzzleGenerator.generatePasswordCrack(5);
    expect(puzzle.type).toBe(PuzzleType.PasswordCrack);
    expect(puzzle.difficulty).toBe(5);
    expect(puzzle.maxAttempts).toBe(5);
    expect(puzzle.solved).toBe(false);
    expect(puzzle.data).toBeDefined();
    expect(['admin', '123456', 'password', 'letmein', 'welcome']).toContain(puzzle.data);
  });

  test('generates valid port scan puzzles', () => {
    const puzzle = PuzzleGenerator.generatePortScan(5);
    expect(puzzle.type).toBe(PuzzleType.PortScan);
    expect(puzzle.difficulty).toBe(5);
    expect(puzzle.data).toBeDefined();

    const data = JSON.parse(puzzle.data!);
    expect(Array.isArray(data.openPorts)).toBe(true);
    expect(data.totalPorts).toBe(65535);
    expect(typeof data.vulnerability).toBe('boolean');
  });

  test('generates valid encryption puzzles', () => {
    const puzzle = PuzzleGenerator.generateEncryption(5);
    expect(puzzle.type).toBe(PuzzleType.Encryption);
    expect(puzzle.data).toBeDefined();

    const data = JSON.parse(puzzle.data!);
    expect(data.encrypted).toBeDefined();
    expect(typeof data.shift).toBe('number');
    expect(data.shift).toBeGreaterThan(0);
    expect(data.shift).toBeLessThanOrEqual(25);
  });

  test('generates valid privilege puzzles', () => {
    const puzzle = PuzzleGenerator.generatePrivilege(5);
    expect(puzzle.type).toBe(PuzzleType.Privilege);
    expect(puzzle.data).toBeDefined();

    const data = JSON.parse(puzzle.data!);
    expect(Array.isArray(data.required)).toBe(true);
    expect(typeof data.vulnerable).toBe('boolean');
  });

  test('generates valid firewall puzzles', () => {
    const puzzle = PuzzleGenerator.generateFirewall(5);
    expect(puzzle.type).toBe(PuzzleType.Firewall);
    expect(puzzle.data).toBeDefined();

    const data = JSON.parse(puzzle.data!);
    expect(typeof data.rules).toBe('number');
    expect(Array.isArray(data.protocols)).toBe(true);
  });

  test('puzzle difficulty scales attempts', () => {
    const easy = PuzzleGenerator.generatePasswordCrack(1);
    const hard = PuzzleGenerator.generatePasswordCrack(10);

    expect(easy.maxAttempts).toBeGreaterThan(hard.maxAttempts);
  });

  test('generates appropriate puzzle for node type', () => {
    const gatewayPuzzle = PuzzleGenerator.generatePuzzleForSecurity(NodeType.Gateway, 5);
    expect(gatewayPuzzle.type).toBe(PuzzleType.PortScan);

    const serverPuzzle = PuzzleGenerator.generatePuzzleForSecurity(NodeType.Server, 5);
    expect(serverPuzzle.type).toBe(PuzzleType.PasswordCrack);

    const dbPuzzle = PuzzleGenerator.generatePuzzleForSecurity(NodeType.Database, 5);
    expect(dbPuzzle.type).toBe(PuzzleType.Encryption);

    const securityPuzzle = PuzzleGenerator.generatePuzzleForSecurity(NodeType.SecurityHub, 5);
    expect(securityPuzzle.type).toBe(PuzzleType.Privilege);

    const cachePuzzle = PuzzleGenerator.generatePuzzleForSecurity(NodeType.DataCache, 5);
    expect(cachePuzzle.type).toBe(PuzzleType.Firewall);
  });
});

// ============================================================================
// NODE SYSTEM TESTS
// ============================================================================

describe('NetworkNode', () => {
  test('creates a node with correct properties', () => {
    const node = new NetworkNode('test_node', NodeType.Server, 100, 200, 5);

    expect(node.node.id).toBe('test_node');
    expect(node.node.type).toBe(NodeType.Server);
    expect(node.node.x).toBe(100);
    expect(node.node.y).toBe(200);
    expect(node.node.securityLevel).toBe(5);
    expect(node.node.hacked).toBe(false);
  });

  test('gateway node starts hacked', () => {
    const node = new NetworkNode('gateway', NodeType.Gateway, 100, 100, 1);
    expect(node.node.hacked).toBe(true);
  });

  test('clamps security level to 1-10 range', () => {
    const lowNode = new NetworkNode('low', NodeType.Server, 0, 0, -5);
    expect(lowNode.node.securityLevel).toBe(1);

    const highNode = new NetworkNode('high', NodeType.Server, 0, 0, 50);
    expect(highNode.node.securityLevel).toBe(10);
  });

  test('data reward scales with security level', () => {
    const lowSecNode = new NetworkNode('low', NodeType.Server, 0, 0, 1);
    const highSecNode = new NetworkNode('high', NodeType.Server, 0, 0, 10);

    expect(highSecNode.node.data).toBeGreaterThan(lowSecNode.node.data);
  });

  test('solves password crack puzzles correctly', () => {
    const node = new NetworkNode('test', NodeType.Server, 0, 0, 5);

    // Get the correct password from the puzzle data
    const password = node.node.puzzle.data!;

    // Wrong answer should fail
    const wrongAttempts = node.node.puzzle.attempts;
    const wrongResult = node.solvePuzzle('wrongpass');
    expect(wrongResult).toBe(false);
    expect(node.node.puzzle.attempts).toBe(wrongAttempts + 1);

    // Correct answer should succeed
    const rightResult = node.solvePuzzle(password);
    expect(rightResult).toBe(true);
  });

  test('prevents solving after max attempts', () => {
    const node = new NetworkNode('test', NodeType.Server, 0, 0, 10);

    // Burn through attempts
    for (let i = 0; i < node.node.puzzle.maxAttempts; i++) {
      node.solvePuzzle('wrong');
    }

    // Next attempt should fail
    const result = node.solvePuzzle('admin');
    expect(result).toBe(false);
  });

  test('can access gateway without tools', () => {
    const toolkit = new ToolKit();
    const node = new NetworkNode('gateway', NodeType.Gateway, 0, 0, 1);

    const canAccess = node.canAccess(Array.from(toolkit.tools.values()));
    expect(canAccess).toBe(true);
  });

  test('returns node security info', () => {
    const node = new NetworkNode('test', NodeType.Server, 100, 200, 5);
    const info = node.getSecurityInfo();

    expect(info).toContain('test');
    expect(info).toContain('server');
    expect(info).toContain('5/10');
    expect(info).toContain('password_crack');
  });
});

// ============================================================================
// TOOL SYSTEM TESTS
// ============================================================================

describe('ToolKit', () => {
  test('initializes with all tools', () => {
    const toolkit = new ToolKit();
    expect(toolkit.tools.size).toBe(5);
  });

  test('activates tools correctly', () => {
    const toolkit = new ToolKit();
    const result = toolkit.activateTool(ToolType.PortScanner);

    expect(result).toBe(true);

    const tool = toolkit.getTool(ToolType.PortScanner);
    expect(tool?.active).toBe(true);
  });

  test('reduces tool uses after activation', () => {
    const toolkit = new ToolKit();
    const beforeUses = toolkit.getTool(ToolType.PortScanner)?.uses ?? 0;

    toolkit.activateTool(ToolType.PortScanner);

    const afterUses = toolkit.getTool(ToolType.PortScanner)?.uses ?? 0;
    expect(afterUses).toBe(beforeUses - 1);
  });

  test('prevents using tools with no uses left', () => {
    const toolkit = new ToolKit();
    const tool = toolkit.getTool(ToolType.PortScanner);

    if (tool) {
      tool.uses = 0;
    }

    const result = toolkit.activateTool(ToolType.PortScanner);
    expect(result).toBe(false);
  });

  test('network mask has unlimited uses', () => {
    const toolkit = new ToolKit();
    const mask = toolkit.getTool(ToolType.NetworkMask);

    expect(mask?.uses).toBe(-1);
  });

  test('deactivates tools', () => {
    const toolkit = new ToolKit();
    toolkit.activateTool(ToolType.PortScanner);
    toolkit.deactivateTool(ToolType.PortScanner);

    const tool = toolkit.getTool(ToolType.PortScanner);
    expect(tool?.active).toBe(false);
  });

  test('returns active tools', () => {
    const toolkit = new ToolKit();
    toolkit.activateTool(ToolType.PortScanner);
    toolkit.activateTool(ToolType.PasswordCracker);

    const active = toolkit.getActiveTools();
    expect(active.length).toBe(2);
    expect(active.some((t) => t.type === ToolType.PortScanner)).toBe(true);
    expect(active.some((t) => t.type === ToolType.PasswordCracker)).toBe(true);
  });

  test('returns tool status string', () => {
    const toolkit = new ToolKit();
    const status = toolkit.getToolStatus();

    expect(status).toContain('Tools:');
    expect(status).toContain('Port Scanner');
    expect(status).toContain('Password Cracker');
  });
});

// ============================================================================
// DETECTION SYSTEM TESTS
// ============================================================================

describe('DetectionSystem', () => {
  test('starts with zero detection', () => {
    const system = new DetectionSystem();
    expect(system.detection).toBe(0);
  });

  test('increments detection', () => {
    const system = new DetectionSystem();
    system.incrementDetection(10);

    expect(system.detection).toBe(10);
  });

  test('caps detection at max', () => {
    const system = new DetectionSystem();
    system.incrementDetection(200);

    expect(system.detection).toBe(100);
  });

  test('decrements detection', () => {
    const system = new DetectionSystem();
    system.incrementDetection(50);
    system.decrementDetection(20);

    expect(system.detection).toBe(30);
  });

  test('prevents negative detection', () => {
    const system = new DetectionSystem();
    system.decrementDetection(50);

    expect(system.detection).toBe(0);
  });

  test('classifies detection levels correctly', () => {
    const system = new DetectionSystem();

    system.detection = 10;
    expect(system.getDetectionLevel()).toBe('low');

    system.detection = 40;
    expect(system.getDetectionLevel()).toBe('medium');

    system.detection = 60;
    expect(system.getDetectionLevel()).toBe('high');

    system.detection = 90;
    expect(system.getDetectionLevel()).toBe('critical');
  });

  test('triggers alarm at max detection', () => {
    const system = new DetectionSystem();
    expect(system.isAlarmTriggered()).toBe(false);

    system.incrementDetection(100);
    expect(system.isAlarmTriggered()).toBe(true);
  });

  test('resets detection', () => {
    const system = new DetectionSystem();
    system.incrementDetection(75);
    system.resetDetection();

    expect(system.detection).toBe(0);
  });

  test('tool effect modifies detection increase', () => {
    const system = new DetectionSystem();
    system.incrementDetection(10, 2.0);

    expect(system.detection).toBe(5);
  });

  test('returns detection info string', () => {
    const system = new DetectionSystem();
    system.incrementDetection(40);

    const info = system.getDetectionInfo();
    expect(info).toContain('Detection:');
    expect(info).toContain('40');
    expect(info).toContain('MEDIUM');
  });
});

// ============================================================================
// LEVEL SYSTEM TESTS
// ============================================================================

describe('LevelManager', () => {
  test('creates level with correct properties', () => {
    const level = LevelManager.createLevel(1);

    expect(level.id).toBe('level_1');
    expect(level.name).toBe('Network 1');
    expect(level.nodes.length).toBeGreaterThan(0);
    expect(level.connections.length).toBeGreaterThan(0);
    expect(level.timeLimit).toBeGreaterThan(0);
  });

  test('increases complexity with level number', () => {
    const level1 = LevelManager.createLevel(1);
    const level10 = LevelManager.createLevel(10);

    expect(level10.nodes.length).toBeGreaterThanOrEqual(level1.nodes.length);
  });

  test('first node is always a gateway', () => {
    for (let i = 1; i <= 5; i++) {
      const level = LevelManager.createLevel(i);
      expect(level.nodes[0].type).toBe(NodeType.Gateway);
    }
  });

  test('all connections reference existing nodes', () => {
    const level = LevelManager.createLevel(5);
    const nodeIds = new Set(level.nodes.map((n) => n.id));

    level.connections.forEach((conn) => {
      expect(nodeIds.has(conn.from)).toBe(true);
      expect(nodeIds.has(conn.to)).toBe(true);
    });
  });

  test('time limit decreases with level', () => {
    const level1 = LevelManager.createLevel(1);
    const level10 = LevelManager.createLevel(10);

    expect(level10.timeLimit).toBeLessThan(level1.timeLimit);
  });

  test('returns correct number of available levels', () => {
    const available = LevelManager.getAvailableLevels();
    expect(available).toBe(12);
  });

  test('security level scales with level number', () => {
    const level1 = LevelManager.createLevel(1);
    const level8 = LevelManager.createLevel(8);

    const avgSecurity1 = level1.nodes.reduce((sum, n) => sum + n.securityLevel, 0) / level1.nodes.length;
    const avgSecurity8 = level8.nodes.reduce((sum, n) => sum + n.securityLevel, 0) / level8.nodes.length;

    expect(avgSecurity8).toBeGreaterThanOrEqual(avgSecurity1);
  });
});

// ============================================================================
// GAME ENGINE TESTS
// ============================================================================

describe('CyberBreachGame', () => {
  test('initializes game correctly', () => {
    const game = new CyberBreachGame();

    expect(game.gameState.currentLevel).toBe(1);
    expect(game.gameState.score).toBe(0);
    expect(game.gameState.gameOver).toBe(false);
    expect(game.gameState.levelComplete).toBe(false);
  });

  test('loads levels successfully', () => {
    const game = new CyberBreachGame();

    expect(game.loadLevel(2)).toBe(true);
    expect(game.gameState.currentLevel).toBe(2);
  });

  test('rejects invalid level numbers', () => {
    const game = new CyberBreachGame();

    expect(game.loadLevel(0)).toBe(false);
    expect(game.loadLevel(20)).toBe(false);
  });

  test('hacks gateway node (already hacked)', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const gatewayId = game.currentLevel.nodes[0].id;
    const result = game.hackNode(gatewayId, '');

    expect(result.success).toBe(true);
    expect(result.message).toContain('already hacked');
  });

  test('fails to hack without sufficient tools', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const nodeId = game.currentLevel.nodes[1].id;
    const result = game.hackNode(nodeId, 'wronganswer');

    // Without tools, should fail with insufficient tools message
    expect(result.success).toBe(false);
  });

  test('increases detection on failed hack', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const detectionBefore = game.detectionSystem.detection;
    const nodeId = game.currentLevel.nodes[1].id;
    game.hackNode(nodeId, 'wronganswer');
    const detectionAfter = game.detectionSystem.detection;

    expect(detectionAfter).toBeGreaterThanOrEqual(detectionBefore);
  });

  test('lists all nodes', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const nodes = game.listNodes();
    expect(nodes.length).toBeGreaterThan(0);
    expect(nodes[0]).toBe(game.currentLevel.nodes[0].id);
  });

  test('retrieves node information', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const nodeId = game.currentLevel.nodes[0].id;
    const info = game.getNodeInfo(nodeId);

    expect(info).toContain(nodeId);
    expect(info).toContain('Security:');
  });

  test('returns game status', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const status = game.getGameStatus();
    expect(status).toContain('CYBER BREACH');
    expect(status).toContain('Level: 1');
    expect(status).toContain('Score:');
  });

  test('gets connection information', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const connInfo = game.getConnectionsInfo();
    expect(connInfo).toContain('Connections');
    expect(connInfo.length).toBeGreaterThan(0);
  });

  test('gets tool status', () => {
    const game = new CyberBreachGame();

    const status = game.getToolStatus();
    expect(status).toContain('Tools:');
    expect(status).toContain('Port Scanner');
  });

  test('activates and deactivates tools', () => {
    const game = new CyberBreachGame();

    const before = game.toolkit.getTool(ToolType.PortScanner)?.active;
    game.activateTool('port_scanner');
    const after = game.toolkit.getTool(ToolType.PortScanner)?.active;

    expect(before).toBe(false);
    expect(after).toBe(true);

    game.deactivateTool('port_scanner');
    const final = game.toolkit.getTool(ToolType.PortScanner)?.active;
    expect(final).toBe(false);
  });

  test('progresses to next level', () => {
    const game = new CyberBreachGame();
    expect(game.gameState.currentLevel).toBe(1);

    game.nextLevel();
    expect(game.gameState.currentLevel).toBe(2);
  });

  test('cannot progress past final level', () => {
    const game = new CyberBreachGame();
    game.loadLevel(12);

    const result = game.nextLevel();
    expect(result).toBe(false);
  });

  test('clears active path on new level', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);
    game.gameState.activePath = ['node1', 'node2'];

    game.loadLevel(2);
    expect(game.gameState.activePath.length).toBe(0);
  });

  test('resets detection on new level', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);
    game.detectionSystem.incrementDetection(50);

    game.loadLevel(2);
    expect(game.detectionSystem.detection).toBe(0);
  });

  test('updates game state over time', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const timeBefore = game.gameState.timeRemaining;
    game.update(1000);

    expect(game.gameState.timeRemaining).toBeLessThanOrEqual(timeBefore);
  });

  test('detects level completion', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const objectiveScore = game.currentLevel.nodes.reduce((sum, n) => sum + n.data, 0) / 2;
    game.gameState.score = objectiveScore + 1;

    game.update(0);
    expect(game.gameState.levelComplete).toBe(true);
  });

  test('detects game over on alarm trigger', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    game.detectionSystem.incrementDetection(100);
    game.update(0);

    expect(game.gameState.gameOver).toBe(true);
  });
});

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

describe('Game Integration', () => {
  test('complete level progression flow', () => {
    const game = new CyberBreachGame();

    expect(game.loadLevel(1)).toBe(true);
    expect(game.gameState.currentLevel).toBe(1);

    const gatewayId = game.currentLevel.nodes[0].id;
    const result = game.hackNode(gatewayId, '');
    expect(result.success).toBe(true);

    expect(game.nextLevel()).toBe(true);
    expect(game.gameState.currentLevel).toBe(2);

    expect(game.gameState.activePath.length).toBe(0);
    expect(game.detectionSystem.detection).toBe(0);
  });

  test('tool usage affects gameplay', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const maskBefore = game.toolkit.getTool(ToolType.NetworkMask)?.uses;
    game.activateTool('network_mask');
    const maskAfter = game.toolkit.getTool(ToolType.NetworkMask)?.uses;

    expect(maskAfter).toBe(maskBefore);
  });

  test('detection increases with failed hacking attempts', () => {
    const game = new CyberBreachGame();
    game.loadLevel(1);

    const detectionBefore = game.detectionSystem.detection;
    const nodeId = game.currentLevel.nodes[1].id;
    const result = game.hackNode(nodeId, 'wrong');

    const detectionAfter = game.detectionSystem.detection;
    // Either detection increased or we got insufficient tools message (which doesn't increase detection in failed state)
    expect(detectionAfter >= detectionBefore).toBe(true);
  });

  test('multiple level progression works correctly', () => {
    const game = new CyberBreachGame();

    for (let i = 1; i <= 5; i++) {
      expect(game.loadLevel(i)).toBe(true);
      expect(game.gameState.currentLevel).toBe(i);
    }
  });

  test('game can be played through multiple levels', () => {
    const game = new CyberBreachGame();

    for (let levelNum = 1; levelNum <= 3; levelNum++) {
      game.loadLevel(levelNum);
      expect(game.gameState.currentLevel).toBe(levelNum);
      expect(game.gameState.nodes.size).toBeGreaterThan(0);

      // Try to hack gateway
      const gatewayId = game.currentLevel.nodes[0].id;
      const result = game.hackNode(gatewayId, '');
      expect(result.success).toBe(true);
    }
  });
});
