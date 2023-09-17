use autocxx::prelude::*;

include_cpp! {
    // C++ headers we want to include.
    #include "faiss/IndexBinary.h"
    #include "faiss/IndexBinaryFlat.h"
    #include "faiss/IndexBinaryHash.h"
    // #include "faiss/IndexBinaryHNSW.h"
    #include "faiss/IndexBinaryIVF.h"
    #include "faiss/MetricType.h"
    #include "faiss/IndexIDMap.h"
    // Safety policy. We are marking that this whole C++ inclusion is unsafe
    // which means the functions themselves do not need to be marked
    // as unsafe. Other policies are possible.
    safety!(unsafe)

    name!(faiss_ffi)
    // What types and functions we want to generate
    generate_ns!("faiss")
}

pub use faiss_ffi::faiss::*;