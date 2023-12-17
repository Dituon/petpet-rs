import moe.d2n.petpetrs.PetpetJNILoader;
import moe.d2n.petpetrs.PetpetRsBuilder;
import moe.d2n.petpetrs.PetpetRsService;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;

public class BaseTest {
    static {
        PetpetJNILoader.loadLibrary("./", "petpet");
        System.out.println("load library success");
    }

    static Path path = Paths.get("F:\\dev\\petpet-rs\\data");

    static String testAvatarUrl = "https://avatars.githubusercontent.com/u/68615161?v=4";
    static String testText = "Petpet";

    static PetpetRsBuilder.AvatarUrlsData urlsData = new PetpetRsBuilder.AvatarUrlsData(
            testAvatarUrl, testAvatarUrl, testAvatarUrl, testAvatarUrl, null
    );
    static PetpetRsBuilder.TextData textData = new PetpetRsBuilder.TextData(
            testText, testText, testText, new String[]{testText}
    );

    @Test
    public void testService() throws IOException {
        PetpetRsService service = PetpetRsService.withRootPath(path);
        service.close();
    }

    @Test
    public void testBuilder() throws IOException {
        PetpetRsService service = PetpetRsService.withRootPath(path);
        PetpetRsBuilder builder = service.getBuilder("gluing");
        PetpetRsBuilder.Result result = builder.build(urlsData, textData);
        assert result.bytes.length != 0;
        service.close();
    }
}
