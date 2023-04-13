use vtx_jbridge::class;

class!(
    package java.nio.file;

    public class Path {
        static "of", "(Ljava/lang/String;[Ljava/lang/String;)Ljava/nio/file/Path;" {
            Argument::new(0, MethodType::Void)
        }
    }
);
