use vtx_jbridge::class;

class!(
    package java.lang;

    public class Object {
        //method takes the same arguments as Frame::exec
        static "<init>", "()V" {
            Argument::new(0, MethodType::Void)
        }
    }
);
