// Time Loop Detective - Core Game Engine
// A mystery-solving game with time loop mechanics

// Types and Interfaces
export interface TimeSlot {
  hour: number;
  minute: number;
}

export interface RoutineBlock {
  timeStart: TimeSlot;
  timeEnd: TimeSlot;
  location: string;
  activity: string;
}

export interface DialogueOption {
  text: string;
  condition?: (evidence: Map<string, Evidence>) => boolean;
  response: string;
  giveEvidence?: string;
  npcReaction?: string;
}

export interface DialogueTree {
  [key: string]: {
    greeting: string;
    topic: string;
    options: DialogueOption[];
    hasGivenEvidence?: Set<string>;
  };
}

export interface Evidence {
  id: string;
  name: string;
  description: string;
  location: string;
  requiredTime: TimeSlot;
  prerequisites: string[];
  clueValue: string;
}

export interface NPCCharacter {
  id: string;
  name: string;
  age: number;
  role: string;
  personality: string;
  routine: RoutineBlock[];
  dialogue: DialogueTree;
  suspicion: number;
  knowsAboutLoop?: boolean;
}

export interface GameState {
  currentTime: TimeSlot;
  loopCount: number;
  inventory: Map<string, Evidence>;
  visitedLocations: Set<string>;
  npcInteractions: Map<string, number>;
  conversationFlags: Map<string, boolean>;
  gameEnded: boolean;
  endingType?: string;
}

// Time utilities
export class TimeManager {
  static readonly MINUTES_PER_SLOT = 15;
  static readonly GAME_START_TIME = { hour: 6, minute: 0 };
  static readonly GAME_END_TIME = { hour: 6, minute: 0 };

  static minutesToSlots(minutes: number): number {
    return Math.floor(minutes / this.MINUTES_PER_SLOT);
  }

  static slotsToMinutes(slots: number): number {
    return slots * this.MINUTES_PER_SLOT;
  }

  static timeToMinutes(time: TimeSlot): number {
    return time.hour * 60 + time.minute;
  }

  static minutesToTime(minutes: number): TimeSlot {
    const adjustedMinutes = minutes % (24 * 60);
    return {
      hour: Math.floor(adjustedMinutes / 60) % 24,
      minute: adjustedMinutes % 60,
    };
  }

  static advanceTime(time: TimeSlot, slots: number): TimeSlot {
    const minutes = this.timeToMinutes(time) + slots * this.MINUTES_PER_SLOT;
    return this.minutesToTime(minutes);
  }

  static formatTime(time: TimeSlot): string {
    const hour = String(time.hour).padStart(2, '0');
    const minute = String(time.minute).padStart(2, '0');
    return `${hour}:${minute}`;
  }

  static isTimeInRange(
    time: TimeSlot,
    startTime: TimeSlot,
    endTime: TimeSlot
  ): boolean {
    const timeMinutes = this.timeToMinutes(time);
    const startMinutes = this.timeToMinutes(startTime);
    const endMinutes = this.timeToMinutes(endTime);

    if (startMinutes <= endMinutes) {
      return timeMinutes >= startMinutes && timeMinutes < endMinutes;
    }

    // Handle overnight ranges
    return timeMinutes >= startMinutes || timeMinutes < endMinutes;
  }
}

// NPC Manager
export class NPCManager {
  private npcs: Map<string, NPCCharacter> = new Map();

  addNPC(npc: NPCCharacter): void {
    this.npcs.set(npc.id, npc);
  }

  getNPC(id: string): NPCCharacter | undefined {
    return this.npcs.get(id);
  }

  getAllNPCs(): NPCCharacter[] {
    return Array.from(this.npcs.values());
  }

  getLocationAtTime(npc: NPCCharacter, time: TimeSlot): string {
    for (const block of npc.routine) {
      if (
        TimeManager.isTimeInRange(time, block.timeStart, block.timeEnd)
      ) {
        return block.location;
      }
    }
    return 'Unknown';
  }

  getActivityAtTime(npc: NPCCharacter, time: TimeSlot): string {
    for (const block of npc.routine) {
      if (
        TimeManager.isTimeInRange(time, block.timeStart, block.timeEnd)
      ) {
        return block.activity;
      }
    }
    return 'Unknown';
  }
}

// Evidence Manager
export class EvidenceManager {
  private evidence: Map<string, Evidence> = new Map();
  private collectedEvidence: Map<string, Evidence> = new Map();

  addEvidence(evidence: Evidence): void {
    this.evidence.set(evidence.id, evidence);
  }

  findEvidenceAtLocation(location: string): Evidence[] {
    return Array.from(this.evidence.values()).filter(
      (e) => e.location === location
    );
  }

  canCollectEvidence(
    evidenceId: string,
    currentTime: TimeSlot,
    inventory: Map<string, Evidence>
  ): boolean {
    const evidence = this.evidence.get(evidenceId);
    if (!evidence) return false;

    // Check time requirement
    if (
      !TimeManager.isTimeInRange(
        currentTime,
        evidence.requiredTime,
        TimeManager.advanceTime(evidence.requiredTime, 1)
      )
    ) {
      return false;
    }

    // Check prerequisites
    for (const prereq of evidence.prerequisites) {
      if (!inventory.has(prereq)) {
        return false;
      }
    }

    return true;
  }

  collectEvidence(evidenceId: string, inventory: Map<string, Evidence>): boolean {
    const evidence = this.evidence.get(evidenceId);
    if (!evidence) return false;

    inventory.set(evidenceId, evidence);
    this.collectedEvidence.set(evidenceId, evidence);
    return true;
  }

  getEvidence(id: string): Evidence | undefined {
    return this.evidence.get(id);
  }
}

// Dialogue Manager
export class DialogueManager {
  private conversationHistory: Map<string, string[]> = new Map();

  startDialogue(npc: NPCCharacter, topic: string): string | null {
    const dialogue = npc.dialogue[topic];
    if (!dialogue) return null;
    return dialogue.greeting;
  }

  getDialogueOptions(
    npc: NPCCharacter,
    topic: string,
    evidence: Map<string, Evidence>
  ): DialogueOption[] {
    const dialogue = npc.dialogue[topic];
    if (!dialogue) return [];

    return dialogue.options.filter((option) => {
      if (option.condition) {
        return option.condition(evidence);
      }
      return true;
    });
  }

  recordConversation(npcId: string, dialogue: string): void {
    if (!this.conversationHistory.has(npcId)) {
      this.conversationHistory.set(npcId, []);
    }
    this.conversationHistory.get(npcId)!.push(dialogue);
  }

  getConversationHistory(npcId: string): string[] {
    return this.conversationHistory.get(npcId) || [];
  }
}

// Main Game Engine
export class TimeLoopDetectiveGame {
  private gameState: GameState;
  private npcManager: NPCManager;
  private evidenceManager: EvidenceManager;
  private dialogueManager: DialogueManager;

  constructor() {
    this.gameState = {
      currentTime: { ...TimeManager.GAME_START_TIME },
      loopCount: 0,
      inventory: new Map(),
      visitedLocations: new Set(),
      npcInteractions: new Map(),
      conversationFlags: new Map(),
      gameEnded: false,
    };

    this.npcManager = new NPCManager();
    this.evidenceManager = new EvidenceManager();
    this.dialogueManager = new DialogueManager();

    this.initializeGame();
  }

  private initializeGame(): void {
    this.createNPCs();
    this.createEvidence();
  }

  private createNPCs(): void {
    // NPC 1: Dr. Helena Crane - The Scientist
    const helenaCrane: NPCCharacter = {
      id: 'helena',
      name: 'Dr. Helena Crane',
      age: 45,
      role: 'Lead Scientist',
      personality: 'Mysterious, stressed, overworking',
      routine: [
        {
          timeStart: { hour: 6, minute: 0 },
          timeEnd: { hour: 7, minute: 0 },
          location: 'Lab',
          activity: 'Preparing experiment',
        },
        {
          timeStart: { hour: 7, minute: 0 },
          timeEnd: { hour: 12, minute: 0 },
          location: 'Lab',
          activity: 'Running experiments',
        },
        {
          timeStart: { hour: 12, minute: 0 },
          timeEnd: { hour: 13, minute: 0 },
          location: 'Cafeteria',
          activity: 'Having lunch',
        },
        {
          timeStart: { hour: 13, minute: 0 },
          timeEnd: { hour: 18, minute: 0 },
          location: 'Lab',
          activity: 'Analyzing data',
        },
        {
          timeStart: { hour: 18, minute: 0 },
          timeEnd: { hour: 20, minute: 0 },
          location: 'Office',
          activity: 'Writing reports',
        },
        {
          timeStart: { hour: 20, minute: 0 },
          timeEnd: { hour: 6, minute: 0 },
          location: 'Home',
          activity: 'Sleeping',
        },
      ],
      dialogue: {
        lab: {
          greeting: 'Helena looks up from her experiments. "Oh, you again."',
          topic: 'experiments',
          options: [
            {
              text: 'What are you working on?',
              response:
                'Helena hesitates. "Temporal physics. The time anomaly... it\'s getting worse." She looks scared.',
              giveEvidence: 'lab-notes',
            },
            {
              text: 'Have you seen anything unusual?',
              condition: (evidence) => evidence.has('damaged-equipment'),
              response:
                'She nods grimly. "Someone sabotaged Equipment Room B. I found burn marks."',
            },
          ],
          hasGivenEvidence: new Set(),
        },
        office: {
          greeting: 'Helena is writing furiously.',
          topic: 'reports',
          options: [
            {
              text: 'What are you reporting?',
              response: 'Helena looks up cautiously. "Safety concerns. I had to document everything."',
            },
          ],
          hasGivenEvidence: new Set(),
        },
      },
      suspicion: 35,
    };

    // NPC 2: Marcus Chen - The Security Guard
    const marcusChen: NPCCharacter = {
      id: 'marcus',
      name: 'Marcus Chen',
      age: 38,
      role: 'Security Officer',
      personality: 'Observant, loyal, protective',
      routine: [
        {
          timeStart: { hour: 6, minute: 0 },
          timeEnd: { hour: 9, minute: 0 },
          location: 'Security Office',
          activity: 'Morning briefing and patrols',
        },
        {
          timeStart: { hour: 9, minute: 0 },
          timeEnd: { hour: 12, minute: 0 },
          location: 'Building A',
          activity: 'Patrolling',
        },
        {
          timeStart: { hour: 12, minute: 0 },
          timeEnd: { hour: 13, minute: 0 },
          location: 'Cafeteria',
          activity: 'Having lunch',
        },
        {
          timeStart: { hour: 13, minute: 0 },
          timeEnd: { hour: 17, minute: 0 },
          location: 'Building B',
          activity: 'Patrolling',
        },
        {
          timeStart: { hour: 17, minute: 0 },
          timeEnd: { hour: 18, minute: 0 },
          location: 'Security Office',
          activity: 'Evening briefing',
        },
        {
          timeStart: { hour: 18, minute: 0 },
          timeEnd: { hour: 6, minute: 0 },
          location: 'Home',
          activity: 'Sleeping',
        },
      ],
      dialogue: {
        security: {
          greeting: 'Marcus looks alert. "Everything okay? You look distressed."',
          topic: 'investigation',
          options: [
            {
              text: 'Have you seen anything suspicious?',
              response: 'Marcus leans in. "There\'s been unusual access logs. Someone entered the restricted lab at 3 AM."',
              giveEvidence: 'access-logs',
            },
            {
              text: 'Can I see the security footage?',
              condition: (evidence) => evidence.has('access-logs'),
              response:
                'Marcus hesitates, then nods. "I can get you footage from the main corridor. Frame 2847 shows someone in dark clothes."',
              giveEvidence: 'footage-frame',
            },
          ],
          hasGivenEvidence: new Set(),
        },
      },
      suspicion: 15,
    };

    // NPC 3: Dr. Richard Stone - The Ambitious Rival
    const richardStone: NPCCharacter = {
      id: 'richard',
      name: 'Dr. Richard Stone',
      age: 52,
      role: 'Research Director',
      personality: 'Arrogant, ambitious, calculating',
      routine: [
        {
          timeStart: { hour: 6, minute: 0 },
          timeEnd: { hour: 8, minute: 0 },
          location: 'Office',
          activity: 'Reviewing proposals',
        },
        {
          timeStart: { hour: 8, minute: 0 },
          timeEnd: { hour: 11, minute: 0 },
          location: 'Meeting Room',
          activity: 'Team meetings',
        },
        {
          timeStart: { hour: 11, minute: 0 },
          timeEnd: { hour: 12, minute: 0 },
          location: 'Equipment Room A',
          activity: 'Inspecting equipment',
        },
        {
          timeStart: { hour: 12, minute: 0 },
          timeEnd: { hour: 13, minute: 0 },
          location: 'Cafeteria',
          activity: 'Networking',
        },
        {
          timeStart: { hour: 13, minute: 0 },
          timeEnd: { hour: 17, minute: 0 },
          location: 'Office',
          activity: 'Strategic planning',
        },
        {
          timeStart: { hour: 17, minute: 0 },
          timeEnd: { hour: 6, minute: 0 },
          location: 'Home',
          activity: 'Sleeping',
        },
      ],
      dialogue: {
        office: {
          greeting: 'Richard looks up coldly. "What do you want?"',
          topic: 'research',
          options: [
            {
              text: 'Who would sabotage Helena\'s research?',
              response: 'Richard smirks. "Plenty of people. Her work threatens the status quo. Why does it matter?"',
            },
            {
              text: 'I found your notebook in the lab at 3 AM',
              condition: (evidence) => evidence.has('richard-notebook'),
              response:
                'Richard\'s face goes pale. "That\'s... a misunderstanding. I was reviewing Helena\'s methods."',
            },
          ],
          hasGivenEvidence: new Set(),
        },
      },
      suspicion: 70,
    };

    // NPC 4: Emma Walters - The Lab Technician
    const emmaWalters: NPCCharacter = {
      id: 'emma',
      name: 'Emma Walters',
      age: 26,
      role: 'Laboratory Technician',
      personality: 'Anxious, helpful, loyal to Helena',
      routine: [
        {
          timeStart: { hour: 6, minute: 0 },
          timeEnd: { hour: 7, minute: 0 },
          location: 'Lab',
          activity: 'Preparing materials',
        },
        {
          timeStart: { hour: 7, minute: 0 },
          timeEnd: { hour: 12, minute: 0 },
          location: 'Lab',
          activity: 'Assisting with experiments',
        },
        {
          timeStart: { hour: 12, minute: 0 },
          timeEnd: { hour: 13, minute: 0 },
          location: 'Cafeteria',
          activity: 'Eating quickly',
        },
        {
          timeStart: { hour: 13, minute: 0 },
          timeEnd: { hour: 17, minute: 0 },
          location: 'Equipment Room B',
          activity: 'Maintaining equipment',
        },
        {
          timeStart: { hour: 17, minute: 0 },
          timeEnd: { hour: 6, minute: 0 },
          location: 'Home',
          activity: 'Sleeping',
        },
      ],
      dialogue: {
        lab: {
          greeting: 'Emma looks nervous. "Hi... did you need something?"',
          topic: 'workplace',
          options: [
            {
              text: 'How do you get along with Dr. Stone?',
              response:
                'Emma frowns. "He\'s... demanding. He was here late yesterday. I saw him near the restricted equipment."',
              giveEvidence: 'emma-statement',
            },
            {
              text: 'Did you see who damaged the equipment?',
              condition: (evidence) => evidence.has('emma-statement'),
              response:
                'Emma whispers, "I think... I think it might have been Dr. Stone. The timing is suspicious."',
            },
          ],
          hasGivenEvidence: new Set(),
        },
      },
      suspicion: 5,
    };

    // NPC 5: Dr. James Patterson - The Director
    const jamesPatterson: NPCCharacter = {
      id: 'james',
      name: 'Dr. James Patterson',
      age: 58,
      role: 'Facility Director',
      personality: 'Diplomatic, stressed, hiding secrets',
      routine: [
        {
          timeStart: { hour: 6, minute: 0 },
          timeEnd: { hour: 7, minute: 0 },
          location: 'Executive Office',
          activity: 'Reading reports',
        },
        {
          timeStart: { hour: 7, minute: 0 },
          timeEnd: { hour: 11, minute: 0 },
          location: 'Meeting Room',
          activity: 'Board meetings',
        },
        {
          timeStart: { hour: 11, minute: 0 },
          timeEnd: { hour: 12, minute: 0 },
          location: 'Restricted Lab',
          activity: 'Reviewing project',
        },
        {
          timeStart: { hour: 12, minute: 0 },
          timeEnd: { hour: 13, minute: 0 },
          location: 'Cafeteria',
          activity: 'Lunch meeting',
        },
        {
          timeStart: { hour: 13, minute: 0 },
          timeEnd: { hour: 18, minute: 0 },
          location: 'Executive Office',
          activity: 'Phone calls and emails',
        },
        {
          timeStart: { hour: 18, minute: 0 },
          timeEnd: { hour: 6, minute: 0 },
          location: 'Home',
          activity: 'Sleeping',
        },
      ],
      dialogue: {
        office: {
          greeting: 'James looks concerned. "I\'ve been expecting you. Time is running out."',
          topic: 'truth',
          options: [
            {
              text: 'What do you know about the time loop?',
              response:
                'James sighs heavily. "The temporal experiment went wrong. Someone tampered with it deliberately."',
              giveEvidence: 'director-confession',
            },
            {
              text: 'Who sabotaged the experiment?',
              condition: (evidence) => evidence.has('richard-notebook') && evidence.has('emma-statement'),
              response:
                'James looks sorrowful. "It was Dr. Stone. He wanted to steal Helena\'s breakthrough. The loop was... an accident."',
            },
          ],
          hasGivenEvidence: new Set(),
        },
      },
      suspicion: 40,
      knowsAboutLoop: true,
    };

    this.npcManager.addNPC(helenaCrane);
    this.npcManager.addNPC(marcusChen);
    this.npcManager.addNPC(richardStone);
    this.npcManager.addNPC(emmaWalters);
    this.npcManager.addNPC(jamesPatterson);
  }

  private createEvidence(): void {
    const evidenceList: Evidence[] = [
      {
        id: 'lab-notes',
        name: 'Helena\'s Lab Notes',
        description: 'Encrypted notes about temporal physics experiments',
        location: 'Lab',
        requiredTime: { hour: 7, minute: 0 },
        prerequisites: [],
        clueValue: 'Reveals the time loop experiment',
      },
      {
        id: 'access-logs',
        name: 'Security Access Logs',
        description: 'Digital record of facility access',
        location: 'Security Office',
        requiredTime: { hour: 9, minute: 0 },
        prerequisites: [],
        clueValue: 'Shows who was in restricted areas',
      },
      {
        id: 'damaged-equipment',
        name: 'Damaged Equipment Report',
        description: 'Photos and analysis of sabotaged equipment',
        location: 'Equipment Room B',
        requiredTime: { hour: 14, minute: 0 },
        prerequisites: [],
        clueValue: 'Proves deliberate sabotage occurred',
      },
      {
        id: 'footage-frame',
        name: 'Security Footage Frame',
        description: 'High-resolution image from corridor camera',
        location: 'Security Office',
        requiredTime: { hour: 10, minute: 0 },
        prerequisites: ['access-logs'],
        clueValue: 'Identifies the saboteur',
      },
      {
        id: 'richard-notebook',
        name: 'Richard\'s Notebook',
        description: 'Handwritten notes detailing research theft plans',
        location: 'Meeting Room',
        requiredTime: { hour: 11, minute: 0 },
        prerequisites: [],
        clueValue: 'Directly implicates Dr. Stone',
      },
      {
        id: 'emma-statement',
        name: 'Emma\'s Witness Statement',
        description: 'Sworn testimony of suspicious activity',
        location: 'Lab',
        requiredTime: { hour: 15, minute: 0 },
        prerequisites: [],
        clueValue: 'Corroborates other evidence against Stone',
      },
      {
        id: 'director-confession',
        name: 'Director\'s Full Confession',
        description: 'Complete account of the sabotage and time loop accident',
        location: 'Executive Office',
        requiredTime: { hour: 8, minute: 0 },
        prerequisites: ['lab-notes'],
        clueValue: 'Explains everything and provides solution',
      },
    ];

    for (const evidence of evidenceList) {
      this.evidenceManager.addEvidence(evidence);
    }
  }

  advanceTime(slots: number): void {
    this.gameState.currentTime = TimeManager.advanceTime(
      this.gameState.currentTime,
      slots
    );

    // Check if loop has ended
    if (
      this.gameState.currentTime.hour === TimeManager.GAME_END_TIME.hour &&
      this.gameState.currentTime.minute === TimeManager.GAME_END_TIME.minute
    ) {
      this.checkLoopBreakingConditions();
      if (!this.gameState.gameEnded) {
        this.resetLoop();
      }
    }
  }

  private checkLoopBreakingConditions(): void {
    const inventory = this.gameState.inventory;

    // Ending 1: Complete Investigation
    if (
      inventory.has('richard-notebook') &&
      inventory.has('footage-frame') &&
      inventory.has('director-confession')
    ) {
      this.gameState.gameEnded = true;
      this.gameState.endingType = 'complete-investigation';
      return;
    }

    // Ending 2: Director's Help
    if (
      inventory.has('director-confession') &&
      inventory.has('damaged-equipment')
    ) {
      this.gameState.gameEnded = true;
      this.gameState.endingType = 'director-assistance';
      return;
    }

    // Ending 3: Direct Confrontation
    if (
      inventory.has('richard-notebook') &&
      inventory.has('emma-statement')
    ) {
      this.gameState.gameEnded = true;
      this.gameState.endingType = 'stone-confrontation';
      return;
    }

    // No ending condition met - loop resets
    this.gameState.endingType = undefined;
  }

  private resetLoop(): void {
    this.gameState.loopCount++;
    this.gameState.currentTime = { ...TimeManager.GAME_START_TIME };
    // Keep inventory and visited locations across loops
  }

  visitLocation(location: string): void {
    this.gameState.visitedLocations.add(location);
  }

  talkToNPC(npcId: string, topic: string): string {
    const npc = this.npcManager.getNPC(npcId);
    if (!npc) return 'NPC not found.';

    const greeting = this.dialogueManager.startDialogue(npc, topic);
    if (!greeting) return 'No conversation available.';

    this.gameState.npcInteractions.set(
      npcId,
      (this.gameState.npcInteractions.get(npcId) || 0) + 1
    );

    return greeting;
  }

  collectEvidenceAtLocation(
    location: string,
    evidenceId: string
  ): boolean {
    const evidence = this.evidenceManager.getEvidence(evidenceId);
    if (!evidence) return false;

    if (evidence.location !== location) return false;

    if (!this.evidenceManager.canCollectEvidence(
      evidenceId,
      this.gameState.currentTime,
      this.gameState.inventory
    )) {
      return false;
    }

    return this.evidenceManager.collectEvidence(
      evidenceId,
      this.gameState.inventory
    );
  }

  getGameState(): GameState {
    return this.gameState;
  }

  getNPCManager(): NPCManager {
    return this.npcManager;
  }

  getEvidenceManager(): EvidenceManager {
    return this.evidenceManager;
  }

  getDialogueManager(): DialogueManager {
    return this.dialogueManager;
  }

  getCurrentTimeString(): string {
    return TimeManager.formatTime(this.gameState.currentTime);
  }

  getLoopStatus(): string {
    return `Loop ${this.gameState.loopCount + 1} - ${this.getCurrentTimeString()}`;
  }

  isGameEnded(): boolean {
    return this.gameState.gameEnded;
  }

  getEndingDescription(): string {
    switch (this.gameState.endingType) {
      case 'complete-investigation':
        return 'You have gathered overwhelming evidence against Dr. Stone. With the notebook, footage, and the director\'s confession, you can break the loop and prevent the sabotage. The timeline resets, and you return to 6:00 AM - but this time, you stop Dr. Stone before the accident ever happens.';
      case 'director-assistance':
        return 'The Director helps you break the temporal paradox using his authority and the evidence of sabotage. The loop collapses, and you awaken outside the time loop\'s influence. The incident is properly investigated and Dr. Stone is arrested.';
      case 'stone-confrontation':
        return 'You confront Dr. Stone with Emma\'s testimony and his own notebook. Caught red-handed, he admits to everything. Security arrives and the temporal field destabilizes - breaking the loop. You escape as the facility goes into lockdown.';
      default:
        return 'The loop continues...';
    }
  }
}
