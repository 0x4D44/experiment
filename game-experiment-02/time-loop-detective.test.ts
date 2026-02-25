// Time Loop Detective - Unit Tests

import {
  TimeManager,
  TimeSlot,
  NPCManager,
  EvidenceManager,
  DialogueManager,
  TimeLoopDetectiveGame,
} from './time-loop-detective';

describe('TimeManager', () => {
  describe('timeToMinutes', () => {
    it('should convert time to minutes correctly', () => {
      expect(TimeManager.timeToMinutes({ hour: 6, minute: 0 })).toBe(360);
      expect(TimeManager.timeToMinutes({ hour: 12, minute: 30 })).toBe(750);
      expect(TimeManager.timeToMinutes({ hour: 23, minute: 59 })).toBe(1439);
    });
  });

  describe('minutesToTime', () => {
    it('should convert minutes to time correctly', () => {
      expect(TimeManager.minutesToTime(360)).toEqual({ hour: 6, minute: 0 });
      expect(TimeManager.minutesToTime(750)).toEqual({ hour: 12, minute: 30 });
    });

    it('should handle wraparound at 24 hours', () => {
      expect(TimeManager.minutesToTime(1440)).toEqual({ hour: 0, minute: 0 });
      expect(TimeManager.minutesToTime(1500)).toEqual({ hour: 1, minute: 0 });
    });
  });

  describe('advanceTime', () => {
    it('should advance time by slots of 15 minutes', () => {
      const start = { hour: 6, minute: 0 };
      const result = TimeManager.advanceTime(start, 1);
      expect(result).toEqual({ hour: 6, minute: 15 });
    });

    it('should advance by multiple slots', () => {
      const start = { hour: 6, minute: 0 };
      const result = TimeManager.advanceTime(start, 4);
      expect(result).toEqual({ hour: 7, minute: 0 });
    });

    it('should handle day wraparound', () => {
      const start = { hour: 23, minute: 45 };
      const result = TimeManager.advanceTime(start, 1);
      expect(result).toEqual({ hour: 0, minute: 0 });
    });
  });

  describe('formatTime', () => {
    it('should format time as HH:MM', () => {
      expect(TimeManager.formatTime({ hour: 6, minute: 0 })).toBe('06:00');
      expect(TimeManager.formatTime({ hour: 12, minute: 30 })).toBe('12:30');
      expect(TimeManager.formatTime({ hour: 9, minute: 5 })).toBe('09:05');
    });
  });

  describe('isTimeInRange', () => {
    it('should correctly identify times in range', () => {
      const startTime = { hour: 9, minute: 0 };
      const endTime = { hour: 12, minute: 0 };

      expect(
        TimeManager.isTimeInRange(
          { hour: 9, minute: 30 },
          startTime,
          endTime
        )
      ).toBe(true);

      expect(
        TimeManager.isTimeInRange(
          { hour: 12, minute: 30 },
          startTime,
          endTime
        )
      ).toBe(false);

      expect(
        TimeManager.isTimeInRange(
          { hour: 9, minute: 0 },
          startTime,
          endTime
        )
      ).toBe(true);
    });

    it('should handle overnight ranges', () => {
      const startTime = { hour: 22, minute: 0 };
      const endTime = { hour: 6, minute: 0 };

      expect(
        TimeManager.isTimeInRange(
          { hour: 23, minute: 0 },
          startTime,
          endTime
        )
      ).toBe(true);

      expect(
        TimeManager.isTimeInRange(
          { hour: 3, minute: 0 },
          startTime,
          endTime
        )
      ).toBe(true);

      expect(
        TimeManager.isTimeInRange(
          { hour: 12, minute: 0 },
          startTime,
          endTime
        )
      ).toBe(false);
    });
  });
});

describe('NPCManager', () => {
  let npcManager: NPCManager;

  beforeEach(() => {
    npcManager = new NPCManager();

    const testNPC = {
      id: 'test-npc',
      name: 'Test Character',
      age: 30,
      role: 'Tester',
      personality: 'Curious',
      routine: [
        {
          timeStart: { hour: 9, minute: 0 },
          timeEnd: { hour: 12, minute: 0 },
          location: 'Office',
          activity: 'Working',
        },
        {
          timeStart: { hour: 12, minute: 0 },
          timeEnd: { hour: 13, minute: 0 },
          location: 'Cafeteria',
          activity: 'Eating',
        },
        {
          timeStart: { hour: 13, minute: 0 },
          timeEnd: { hour: 18, minute: 0 },
          location: 'Office',
          activity: 'Working',
        },
      ],
      dialogue: {},
      suspicion: 50,
    };

    npcManager.addNPC(testNPC);
  });

  describe('getLocationAtTime', () => {
    it('should return correct location for given time', () => {
      const npc = npcManager.getNPC('test-npc');
      if (npc) {
        expect(npcManager.getLocationAtTime(npc, { hour: 10, minute: 0 })).toBe(
          'Office'
        );
        expect(npcManager.getLocationAtTime(npc, { hour: 12, minute: 30 })).toBe(
          'Cafeteria'
        );
      }
    });

    it('should return Unknown for times with no scheduled activity', () => {
      const npc = npcManager.getNPC('test-npc');
      if (npc) {
        expect(npcManager.getLocationAtTime(npc, { hour: 3, minute: 0 })).toBe(
          'Unknown'
        );
      }
    });
  });

  describe('getActivityAtTime', () => {
    it('should return correct activity for given time', () => {
      const npc = npcManager.getNPC('test-npc');
      if (npc) {
        expect(npcManager.getActivityAtTime(npc, { hour: 10, minute: 0 })).toBe(
          'Working'
        );
        expect(npcManager.getActivityAtTime(npc, { hour: 12, minute: 30 })).toBe(
          'Eating'
        );
      }
    });
  });
});

describe('EvidenceManager', () => {
  let evidenceManager: EvidenceManager;

  beforeEach(() => {
    evidenceManager = new EvidenceManager();

    evidenceManager.addEvidence({
      id: 'evidence-1',
      name: 'Test Evidence',
      description: 'A test piece of evidence',
      location: 'Lab',
      requiredTime: { hour: 10, minute: 0 },
      prerequisites: [],
      clueValue: 'Important clue',
    });

    evidenceManager.addEvidence({
      id: 'evidence-2',
      name: 'Follow-up Evidence',
      description: 'Requires first evidence',
      location: 'Office',
      requiredTime: { hour: 11, minute: 0 },
      prerequisites: ['evidence-1'],
      clueValue: 'Confirms evidence-1',
    });
  });

  describe('findEvidenceAtLocation', () => {
    it('should find evidence at specific location', () => {
      const found = evidenceManager.findEvidenceAtLocation('Lab');
      expect(found.length).toBe(1);
      expect(found[0].id).toBe('evidence-1');
    });

    it('should return empty array for location with no evidence', () => {
      const found = evidenceManager.findEvidenceAtLocation('Unknown');
      expect(found.length).toBe(0);
    });
  });

  describe('canCollectEvidence', () => {
    it('should allow collection when prerequisites are met and time is correct', () => {
      const inventory = new Map();
      const canCollect = evidenceManager.canCollectEvidence(
        'evidence-1',
        { hour: 10, minute: 0 },
        inventory
      );
      expect(canCollect).toBe(true);
    });

    it('should prevent collection at wrong time', () => {
      const inventory = new Map();
      const canCollect = evidenceManager.canCollectEvidence(
        'evidence-1',
        { hour: 15, minute: 0 },
        inventory
      );
      expect(canCollect).toBe(false);
    });

    it('should prevent collection without prerequisites', () => {
      const inventory = new Map();
      const canCollect = evidenceManager.canCollectEvidence(
        'evidence-2',
        { hour: 11, minute: 0 },
        inventory
      );
      expect(canCollect).toBe(false);
    });

    it('should allow collection with prerequisites met', () => {
      const inventory = new Map();
      const evidence1 = evidenceManager.getEvidence('evidence-1');
      if (evidence1) {
        inventory.set('evidence-1', evidence1);
      }

      const canCollect = evidenceManager.canCollectEvidence(
        'evidence-2',
        { hour: 11, minute: 0 },
        inventory
      );
      expect(canCollect).toBe(true);
    });
  });

  describe('collectEvidence', () => {
    it('should add evidence to inventory', () => {
      const inventory = new Map();
      const success = evidenceManager.collectEvidence('evidence-1', inventory);
      expect(success).toBe(true);
      expect(inventory.has('evidence-1')).toBe(true);
    });

    it('should return false for non-existent evidence', () => {
      const inventory = new Map();
      const success = evidenceManager.collectEvidence('fake-evidence', inventory);
      expect(success).toBe(false);
    });
  });
});

describe('DialogueManager', () => {
  let dialogueManager: DialogueManager;

  beforeEach(() => {
    dialogueManager = new DialogueManager();
  });

  describe('recordConversation', () => {
    it('should record conversation with NPC', () => {
      dialogueManager.recordConversation('npc-1', 'Hello there!');
      dialogueManager.recordConversation('npc-1', 'How are you?');

      const history = dialogueManager.getConversationHistory('npc-1');
      expect(history.length).toBe(2);
      expect(history[0]).toBe('Hello there!');
      expect(history[1]).toBe('How are you?');
    });
  });

  describe('getConversationHistory', () => {
    it('should return empty array for NPC with no conversations', () => {
      const history = dialogueManager.getConversationHistory('unknown-npc');
      expect(history.length).toBe(0);
    });
  });
});

describe('TimeLoopDetectiveGame', () => {
  let game: TimeLoopDetectiveGame;

  beforeEach(() => {
    game = new TimeLoopDetectiveGame();
  });

  describe('initialization', () => {
    it('should start with correct initial time', () => {
      const state = game.getGameState();
      expect(state.currentTime).toEqual({ hour: 6, minute: 0 });
    });

    it('should start with loop count 0', () => {
      const state = game.getGameState();
      expect(state.loopCount).toBe(0);
    });

    it('should have empty inventory', () => {
      const state = game.getGameState();
      expect(state.inventory.size).toBe(0);
    });

    it('should create 5 NPCs', () => {
      const npcs = game.getNPCManager().getAllNPCs();
      expect(npcs.length).toBe(5);
    });
  });

  describe('advanceTime', () => {
    it('should advance time correctly', () => {
      game.advanceTime(1); // +15 minutes
      expect(game.getGameState().currentTime).toEqual({ hour: 6, minute: 15 });
    });

    it('should advance multiple slots', () => {
      game.advanceTime(4); // +60 minutes
      expect(game.getGameState().currentTime).toEqual({ hour: 7, minute: 0 });
    });

    it('should increment loop count when reaching end of day', () => {
      const startLoop = game.getGameState().loopCount;

      // Advance to end of day
      for (let i = 0; i < 96; i++) {
        // 96 slots = 24 hours
        game.advanceTime(1);
        if (game.getGameState().gameEnded) break;
      }

      expect(game.getGameState().loopCount).toBeGreaterThan(startLoop);
    });
  });

  describe('visitLocation', () => {
    it('should track visited locations', () => {
      game.visitLocation('Lab');
      game.visitLocation('Office');

      const state = game.getGameState();
      expect(state.visitedLocations.has('Lab')).toBe(true);
      expect(state.visitedLocations.has('Office')).toBe(true);
    });
  });

  describe('collectEvidenceAtLocation', () => {
    it('should collect evidence at correct location and time', () => {
      // Advance to 7:00 AM when Helena is in Lab
      game.visitLocation('Lab');
      game.advanceTime(4); // 6:00 AM -> 7:00 AM

      const result = game.collectEvidenceAtLocation('Lab', 'lab-notes');
      expect(result).toBe(true);
      expect(game.getGameState().inventory.has('lab-notes')).toBe(true);
    });

    it('should not collect evidence at wrong location', () => {
      game.visitLocation('Lab');
      game.advanceTime(4); // 7:00 AM

      const result = game.collectEvidenceAtLocation('Office', 'lab-notes');
      expect(result).toBe(false);
      expect(game.getGameState().inventory.has('lab-notes')).toBe(false);
    });
  });

  describe('talkToNPC', () => {
    it('should return dialogue when talking to NPC', () => {
      const response = game.talkToNPC('helena', 'lab');
      expect(response.length).toBeGreaterThan(0);
      expect(response).toContain('Helena');
    });

    it('should track NPC interactions', () => {
      game.talkToNPC('helena', 'lab');
      game.talkToNPC('helena', 'lab');

      const state = game.getGameState();
      expect(state.npcInteractions.get('helena')).toBe(2);
    });
  });

  describe('loop breaking conditions', () => {
    it('should detect complete investigation ending', () => {
      // Manually add required evidence
      const state = game.getGameState();
      const evidence1 = game
        .getEvidenceManager()
        .getEvidence('richard-notebook');
      const evidence2 = game
        .getEvidenceManager()
        .getEvidence('footage-frame');
      const evidence3 = game
        .getEvidenceManager()
        .getEvidence('director-confession');

      if (evidence1) state.inventory.set('richard-notebook', evidence1);
      if (evidence2) state.inventory.set('footage-frame', evidence2);
      if (evidence3) state.inventory.set('director-confession', evidence3);

      // Advance to end of day
      for (let i = 0; i < 96; i++) {
        game.advanceTime(1);
        if (game.isGameEnded()) break;
      }

      expect(game.isGameEnded()).toBe(true);
      expect(game.getGameState().endingType).toBe('complete-investigation');
    });

    it('should detect director assistance ending', () => {
      const state = game.getGameState();
      const evidence1 = game
        .getEvidenceManager()
        .getEvidence('director-confession');
      const evidence2 = game
        .getEvidenceManager()
        .getEvidence('damaged-equipment');

      if (evidence1) state.inventory.set('director-confession', evidence1);
      if (evidence2) state.inventory.set('damaged-equipment', evidence2);

      // Advance to end of day
      for (let i = 0; i < 96; i++) {
        game.advanceTime(1);
        if (game.isGameEnded()) break;
      }

      expect(game.isGameEnded()).toBe(true);
      expect(game.getGameState().endingType).toBe('director-assistance');
    });

    it('should detect stone confrontation ending', () => {
      const state = game.getGameState();
      const evidence1 = game
        .getEvidenceManager()
        .getEvidence('richard-notebook');
      const evidence2 = game
        .getEvidenceManager()
        .getEvidence('emma-statement');

      if (evidence1) state.inventory.set('richard-notebook', evidence1);
      if (evidence2) state.inventory.set('emma-statement', evidence2);

      // Advance to end of day
      for (let i = 0; i < 96; i++) {
        game.advanceTime(1);
        if (game.isGameEnded()) break;
      }

      expect(game.isGameEnded()).toBe(true);
      expect(game.getGameState().endingType).toBe('stone-confrontation');
    });
  });

  describe('getCurrentTimeString', () => {
    it('should return formatted time string', () => {
      expect(game.getCurrentTimeString()).toBe('06:00');
      game.advanceTime(1);
      expect(game.getCurrentTimeString()).toBe('06:15');
    });
  });

  describe('getLoopStatus', () => {
    it('should return current loop status', () => {
      const status = game.getLoopStatus();
      expect(status).toContain('Loop 1');
      expect(status).toContain('06:00');
    });
  });

  describe('getEndingDescription', () => {
    it('should return appropriate ending description', () => {
      const state = game.getGameState();
      const evidence = game
        .getEvidenceManager()
        .getEvidence('richard-notebook');

      if (evidence) {
        state.inventory.set('richard-notebook', evidence);
      }

      state.gameEnded = true;
      state.endingType = 'stone-confrontation';

      const description = game.getEndingDescription();
      expect(description).toContain('Stone');
    });
  });
});
