package moe.d2n.petpetrs;

public final class PetpetJNILoader {
    private static final String PROP_KEY = "java.library.path";

    /**
     * example:
     * <blockquote><pre>
     * PetpetJNILoader.loadLibrary("C:\\Program Files\\petpet.dll"); // in Windows
     *
     * PetpetJNILoader.loadLibrary("/usr/lib/petpet.so"); // in Linux
     *
     * PetpetJNILoader.loadLibrary("/opt/local/lib/petpet.dylib"); // in MacOS
     * </pre></blockquote>
     *
     * @param libraryPath petpet binary path
     */
    static public void loadLibrary(String libraryPath) throws
            SecurityException, UnsatisfiedLinkError, NullPointerException
    {
        System.load(libraryPath);
    }

    /**
     * example:
     * <blockquote><pre>
     * PetpetJNILoader.loadLibrary("C:\\Program Files", "petpet"); // C:\Program Files\petpet.dll in Windows
     *
     * PetpetJNILoader.loadLibrary("/usr/lib", "petpet"); // /usr/lib/petpet.so in Linux
     *
     * PetpetJNILoader.loadLibrary("/opt/local/lib/", "petpet"); // /opt/local/lib/petpet.dylib in MacOS
     * </pre></blockquote>
     *
     * @param libraryDir petpet binary path
     */
    static public void loadLibrary(String libraryDir, String libraryName) throws
            SecurityException, UnsatisfiedLinkError, NullPointerException
    {
        String prev = System.getProperty(PROP_KEY);
        System.setProperty(PROP_KEY, libraryDir);
        System.loadLibrary(libraryName);
        System.setProperty(PROP_KEY, prev);
    }
}
