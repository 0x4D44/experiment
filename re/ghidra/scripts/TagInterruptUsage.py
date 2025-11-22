# Ghidra Python script: TagInterruptUsage
# Adds @interrupt annotations to instructions that execute INT xx.

from ghidra.program.model.listing import CodeUnit
from ghidra.program.model.scalar import Scalar
from ghidra.util.task import TaskMonitor

monitor = currentMonitor if 'currentMonitor' in globals() else TaskMonitor.DUMMY
listing = currentProgram.getListing()
count = 0

instr_iter = listing.getInstructions(True)
while instr_iter.hasNext() and not monitor.isCancelled():
    instr = instr_iter.next()
    if instr.getMnemonicString().upper() != 'INT':
        continue
    ops = instr.getOpObjects(0)
    vector = None
    for op in ops:
        if isinstance(op, Scalar):
            vector = int(op.getValue()) & 0xFF
            break
    if vector is None:
        continue
    note = '@interrupt:INT 0x%02X' % vector
    existing = instr.getComment(CodeUnit.EOL_COMMENT)
    if existing and note in existing:
        continue
    new_comment = (existing + ' ' + note).strip() if existing else note
    instr.setComment(CodeUnit.EOL_COMMENT, new_comment)
    count += 1

print('Annotated %d interrupt instructions' % count)
