# petpet-rs Java Binding

# example

```java
// 从本地加载动态链接库
/*
 * PetpetJNILoader.loadLibrary("C:\\Program Files", "petpet"); // C:\Program Files\petpet.dll in Windows
 *
 * PetpetJNILoader.loadLibrary("/usr/lib", "petpet"); // /usr/lib/petpet.so in Linux
 *
 * PetpetJNILoader.loadLibrary("/opt/local/lib/", "petpet"); // /opt/local/lib/petpet.dylib in MacOS
 */
static {
    PetpetJNILoader.loadLibrary("./", "petpet");
    System.out.println("load library success");
}
```

```java
// 数据目录
static Path path = Paths.get("./data");

// 测试头像
static String testAvatarUrl = "https://avatars.githubusercontent.com/u/68615161?v=4";
// 测试文本
static String testText = "Petpet";

// 构建头像数据
static PetpetRsBuilder.AvatarUrlsData urlsData = new PetpetRsBuilder.AvatarUrlsData(
        testAvatarUrl, testAvatarUrl, testAvatarUrl, testAvatarUrl, null
);

// 构建文本数据
static PetpetRsBuilder.TextData textData = new PetpetRsBuilder.TextData(
        testText, testText, testText, new String[]{testText}
);

public static void main(String[]args) throws IOException {
    // 从数据目录加载服务
    PetpetRsService service = PetpetRsService.withRootPath(path);

    // 通过 key 获取 Builder
    PetpetRsBuilder builder = service.getBuilder("gluing");

    // 生成图像
    PetpetRsBuilder.Result result = builder.build(urlsData, textData);
    assert result.bytes.length != 0;

    // 关闭服务防止内存泄露
    service.close();
}
```

详见 [`src/test/java/BaseTest.java`](./src/test/java/BaseTest.java)