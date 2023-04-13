use vtx_jbridge::class;

class!(
    package java.lang;

    public class Integer {
        superclass java.lang.Number; // i don't actually need to implement this bc it is abstract
                                     // mesqueaks ??
        
        static "parseInt", "(Ljava/lang/String;)I" {
            Argument::new(17, MethodType::Int)
        }
    }
);
