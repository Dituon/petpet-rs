package moe.d2n.petpetrs;

//import org.jetbrains.annotations.NotNull;

/**
 * 不建议继承此类, 可能涉及不安全的指针操作: 使用组合拓展此类功能
 */
public class PetpetRsBuilder implements AutoCloseable {
    protected final long pointer;
    private boolean closed = false;

    protected static native long createBuilder(String template, String path) throws RuntimeException;
    protected static native void closeBuilder(long pointer);

    // TODO: Map Exception
    protected static native byte[] builderBuildByString(long pointer, String data) throws RuntimeException;
    protected static native byte[] builderBuildByObjects(
            long pointer, AvatarUrlsData avatarData, TextData textData
    ) throws RuntimeException;

    public PetpetRsBuilder(String template, String path) {
        this.pointer = createBuilder(template, path);
    }

    public Result build(String data) throws RuntimeException {
        if (this.closed) throw new IllegalStateException("Builder Already closed");
        byte[] bytes = builderBuildByString(this.pointer, data);
        return new Result(bytes, EncodeFormat.PNG);
    }

    /**
     * @param avatarData Not Null
     * @param textData Not Null
     */
    public Result build(AvatarUrlsData avatarData, TextData textData) {
        if (avatarData == null || textData == null) throw new IllegalArgumentException("avatarData and textData must not null");
        byte[] bytes = builderBuildByObjects(this.pointer, avatarData, textData);
        return new Result(bytes, EncodeFormat.PNG);
    }

    @Override
    public void close() {
        this.closed = true;
        closeBuilder(this.pointer);
    }

    public static class AvatarUrlsData {
        public String from;
        public String to;
        public String group;
        public String bot;
        public String[] random;

        public AvatarUrlsData(){
        }

        public AvatarUrlsData(String from, String to, String group, String bot, String[] random) {
            this.from = from;
            this.to = to;
            this.group = group;
            this.bot = bot;
            this.random = random;
        }

        public AvatarUrlsData setFrom(String from) {
            this.from = from;
            return this;
        }

        public AvatarUrlsData setTo(String to) {
            this.to = to;
            return this;
        }

        public AvatarUrlsData setGroup(String group) {
            this.group = group;
            return this;
        }

        public AvatarUrlsData setBot(String bot) {
            this.bot = bot;
            return this;
        }

        public AvatarUrlsData setRandom(String[] random) {
            this.random = random;
            return this;
        }
    }

    public static class TextData {
        public String from;
        public String to;
        public String group;
        public String[] textList;

        public TextData(){
        }

        public TextData(String from, String to, String group, String[] textList) {
            this.from = from;
            this.to = to;
            this.group = group;
            this.textList = textList;
        }

        public TextData setFrom(String from) {
            this.from = from;
            return this;
        }

        public TextData setTo(String to) {
            this.to = to;
            return this;
        }

        public TextData setGroup(String group) {
            this.group = group;
            return this;
        }

        public TextData setTextList(String[] textList) {
            this.textList = textList;
            return this;
        }
    }

    public enum EncodeFormat {
        PNG, GIF
    }

    public static class Result {
        public final byte[] bytes;
        public final EncodeFormat format;

        private Result(byte[] bytes, EncodeFormat format) {
            this.bytes = bytes;
            this.format = format;
        }
    }
}
