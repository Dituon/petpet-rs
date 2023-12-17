package moe.d2n.petpetrs;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;
import java.util.function.BiConsumer;
import java.util.stream.Stream;

public class PetpetRsService implements AutoCloseable {
    public static final String TEMPLATE_FILE_NAME = "data.json";
    protected final Map<String, PetpetRsBuilder> map;

    public PetpetRsService() {
        this.map = new HashMap<>();
    }

    public PetpetRsService(int initialCapacity) {
        this.map = new HashMap<>(initialCapacity);
    }

    public static PetpetRsService withRootPath(Path path) throws IOException {
        try(Stream<Path> pathStream = Files.walk(path).filter(Files::isDirectory)) {
            PetpetRsService service = new PetpetRsService((int) pathStream.count());
            service.readRootPath(path);
            return service;
        }
    }

    /**
     * 读取目录下所有模板
     * @param path root path
     * @throws IOException if file not found or IO error
     */
    public PetpetRsService readRootPath(Path path) throws IOException {
        File file = path.toFile();
        File[] subFiles = file.listFiles();
        if (!file.exists() || subFiles == null){
            throw new FileNotFoundException();
        }
        for (File subFile : subFiles) {
            if (!subFile.isDirectory()) continue;
            try {
                this.readPath(subFile.toPath(), subFile.getName());
            } catch (FileNotFoundException ignored){
                // skip if not found
            }
        }
        return this;
    }

    public PetpetRsService readPath(Path path) throws IOException {
        return this.readPath(path, path.getFileName().toString());
    }

    /**
     * 从目录读取模板
     * @param key key in map
     * @throws IOException file not found or IO error
     */
    public PetpetRsService readPath(Path path, String key) throws IOException {
        String absolute = path.toAbsolutePath().toString();
        Path templatePath = Paths.get(absolute, TEMPLATE_FILE_NAME);
        if (!templatePath.toFile().exists()) throw new FileNotFoundException();
        final String template = Files.readString(templatePath);
        final PetpetRsBuilder builder = new PetpetRsBuilder(template, absolute);
        return this.putBuilder(key, builder);
    }

    public PetpetRsService putBuilder(String key, PetpetRsBuilder builder){
        this.map.put(key, builder);
        return this;
    }

    public PetpetRsBuilder getBuilder(String key) {
        return this.map.get(key);
    }

    public PetpetRsService removeBuilder(String key){
        this.map.remove(key).close();
        return this;
    }

    public PetpetRsService forEach(BiConsumer<? super String, ? super PetpetRsBuilder> action){
        this.map.forEach(action);
        return this;
    }

    @Override
    public void close() {
        this.map.forEach((k, v) -> v.close());
    }
}
