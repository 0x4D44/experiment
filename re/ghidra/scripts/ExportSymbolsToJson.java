// Ghidra script: ExportSymbolsToJson
// Usage: run from Ghidra (GUI or headless) to emit re/artifacts/symbols.json

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.*;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSetView;
import ghidra.program.model.lang.Language;
import ghidra.program.model.lang.CompilerSpec;
import ghidra.program.model.symbol.*;
import ghidra.program.model.scalar.Scalar;
import ghidra.program.model.mem.Memory;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.util.exception.CancelledException;

import java.io.BufferedWriter;
import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.security.MessageDigest;
import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Date;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;
import java.util.Set;

public class ExportSymbolsToJson extends GhidraScript {
    private static final String ENV_REPO_ROOT = "F1GP_REPO_ROOT";

    @Override
    public void run() throws Exception {
        if (currentProgram == null) {
            printerr("No active program");
            return;
        }

        String repoRoot = valueOrEnv("f1gp.repo.root", ENV_REPO_ROOT);
        if (repoRoot == null) {
            repoRoot = System.getProperty("user.dir");
        }
        File outFile = new File(repoRoot, "re/artifacts/symbols.json");
        outFile.getParentFile().mkdirs();

        Listing listing = currentProgram.getListing();
        FunctionManager functionManager = currentProgram.getFunctionManager();
        SymbolTable symbolTable = currentProgram.getSymbolTable();

        List<String> functionJson = new ArrayList<>();
        FunctionIterator funcs = functionManager.getFunctions(true);
        monitor.setMessage("Collecting function metadata");
        while (funcs.hasNext() && !monitor.isCancelled()) {
            Function function = funcs.next();
            functionJson.add(serializeFunction(function, listing, symbolTable));
        }

        StringBuilder sb = new StringBuilder();
        sb.append("{\n");
        sb.append("  \"program_name\": \"").append(escape(currentProgram.getName())).append("\",\n");
        sb.append("  \"language\": \"").append(languageString()).append("\",\n");
        sb.append("  \"binary_hash\": \"sha256:").append(computeProgramSha256()).append("\",\n");
        sb.append("  \"generated_at\": \"").append(timestamp()).append("\",\n");
        sb.append("  \"functions\": [\n");
        for (int i = 0; i < functionJson.size(); i++) {
            sb.append(functionJson.get(i));
            if (i < functionJson.size() - 1) {
                sb.append(",");
            }
            sb.append("\n");
        }
        sb.append("  ]\n");
        sb.append("}\n");

        try (BufferedWriter writer = Files.newBufferedWriter(outFile.toPath(), StandardCharsets.UTF_8)) {
            writer.write(sb.toString());
        }

        println("Exported symbols to " + outFile.getAbsolutePath());
    }

    private String serializeFunction(Function function, Listing listing, SymbolTable symbolTable) throws CancelledException {
        Address entry = function.getEntryPoint();
        AddressSetView body = function.getBody();
        long size = body.getNumAddresses();
        String subsystem = parseSubsystem(function);
        String notes = collectNotes(function);
        Set<String> callers = collectCallers(entry, symbolTable);
        Set<String> ints = collectInterrupts(body, listing);
        String traceId = parseTrace(function);

        StringBuilder sb = new StringBuilder();
        sb.append("    {\n");
        sb.append("      \"name\": \"").append(escape(function.getName())).append("\",\n");
        sb.append("      \"entry_linear\": \"0x").append(Long.toHexString(entry.getOffset()).toUpperCase(Locale.ROOT)).append("\",\n");
        sb.append("      \"size_bytes\": ").append(size).append(",\n");
        sb.append("      \"stack_frame_size\": ").append(function.getStackPurgeSize()).append(",\n");
        sb.append("      \"subsystem\": \"").append(subsystem).append("\",\n");
        sb.append("      \"callers\": ").append(stringArray(callers)).append(",\n");
        sb.append("      \"int_usage\": ").append(stringArray(ints)).append(",\n");
        sb.append("      \"source_recording_id\": \"").append(escape(traceId)).append("\",\n");
        sb.append("      \"notes\": \"").append(escape(notes)).append("\"\n");
        sb.append("    }");
        return sb.toString();
    }

    private Set<String> collectCallers(Address entry, SymbolTable symbolTable) {
        Set<String> callers = new HashSet<>();
        ReferenceManager refManager = currentProgram.getReferenceManager();
        ReferenceIterator refs = refManager.getReferencesTo(entry);
        Listing listing = currentProgram.getListing();
        while (refs.hasNext()) {
            Reference ref = refs.next();
            if (!ref.getReferenceType().isCall()) {
                continue;
            }
            Address from = ref.getFromAddress();
            Function callerFn = listing.getFunctionContaining(from);
            if (callerFn != null) {
                callers.add(callerFn.getName());
            } else {
                callers.add("0x" + Long.toHexString(from.getOffset()));
            }
        }
        return callers;
    }

    private Set<String> collectInterrupts(AddressSetView body, Listing listing) throws CancelledException {
        Set<String> ints = new HashSet<>();
        InstructionIterator it = listing.getInstructions(body, true);
        while (it.hasNext() && !monitor.isCancelled()) {
            Instruction inst = it.next();
            if (!"INT".equalsIgnoreCase(inst.getMnemonicString())) {
                continue;
            }
            Object[] ops = inst.getOpObjects(0);
            for (Object op : ops) {
                if (op instanceof Scalar) {
                    long value = ((Scalar) op).getValue();
                    ints.add(String.format("0x%02X", value & 0xff));
                }
            }
        }
        return ints;
    }

    private String parseSubsystem(Function function) {
        String comment = firstNonNull(function.getComment(), function.getRepeatableComment());
        if (comment == null) {
            return "unknown";
        }
        String[] lines = comment.split("\\r?\\n");
        for (String line : lines) {
            line = line.trim();
            if (line.toLowerCase(Locale.ROOT).startsWith("@subsystem:")) {
                return line.substring(11).trim();
            }
        }
        return "unknown";
    }

    private String parseTrace(Function function) {
        String comment = firstNonNull(function.getComment(), function.getRepeatableComment());
        if (comment == null) {
            return "";
        }
        String[] lines = comment.split("\\r?\\n");
        for (String line : lines) {
            line = line.trim();
            if (line.toLowerCase(Locale.ROOT).startsWith("@trace:")) {
                return line.substring(7).trim();
            }
        }
        return "";
    }

    private String collectNotes(Function function) {
        StringBuilder sb = new StringBuilder();
        String comment = function.getComment();
        if (comment != null) {
            sb.append(comment.trim());
        }
        String repeatable = function.getRepeatableComment();
        if (repeatable != null) {
            if (sb.length() > 0) sb.append(" | ");
            sb.append(repeatable.trim());
        }
        return sb.toString();
    }

    private String stringArray(Set<String> entries) {
        StringBuilder sb = new StringBuilder();
        sb.append("[");
        int idx = 0;
        for (String entry : entries) {
            sb.append("\"").append(escape(entry)).append("\"");
            if (idx < entries.size() - 1) sb.append(", ");
            idx++;
        }
        sb.append("]");
        return sb.toString();
    }

    private String languageString() {
        Language lang = currentProgram.getLanguage();
        CompilerSpec compSpec = currentProgram.getCompilerSpec();
        return lang.getLanguageID().getIdAsString() + " / " + compSpec.getCompilerSpecID();
    }

    private String computeProgramSha256() throws Exception {
        Memory memory = currentProgram.getMemory();
        MessageDigest digest = MessageDigest.getInstance("SHA-256");
        for (MemoryBlock block : memory.getBlocks()) {
            if (!block.isInitialized()) {
                continue;
            }
            byte[] data = new byte[(int) block.getSize()];
            block.getBytes(block.getStart(), data);
            digest.update(data);
        }
        byte[] hash = digest.digest();
        return bytesToHex(hash);
    }

    private String timestamp() {
        return new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'").format(new Date());
    }

    private String valueOrEnv(String propertyKey, String envKey) {
        String value = System.getProperty(propertyKey);
        if (value != null && !value.isEmpty()) {
            return value;
        }
        value = System.getenv(envKey);
        if (value != null && !value.isEmpty()) {
            return value;
        }
        return null;
    }

    private String bytesToHex(byte[] data) {
        StringBuilder sb = new StringBuilder();
        for (byte b : data) {
            sb.append(String.format("%02x", b));
        }
        return sb.toString();
    }

    private String escape(String input) {
        if (input == null) {
            return "";
        }
        return input.replace("\\", "\\\\").replace("\"", "\\\"");
    }

    private String firstNonNull(String a, String b) {
        if (a != null && !a.isEmpty()) {
            return a;
        }
        return (b == null ? null : b);
    }
}
