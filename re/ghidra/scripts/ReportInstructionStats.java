import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.InstructionIterator;

public class ReportInstructionStats extends GhidraScript {
    @Override
    public void run() throws Exception {
        InstructionIterator it = currentProgram.getListing().getInstructions(true);
        int count = 0;
        while (it.hasNext()) {
            it.next();
            count++;
        }
        println("Instruction count: " + count);
    }
}
