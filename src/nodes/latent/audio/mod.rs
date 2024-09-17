//!audio
///**EmptyLatentAudio**
pub struct EmptyLatentAudio<Seconds: crate::nodes::Float> {
    ///No documentation.
    pub seconds: Seconds,
}
///**VAEDecodeAudio**
pub struct VaeDecodeAudio<Samples: crate::nodes::Latent, Vae: crate::nodes::Vae> {
    ///No documentation.
    pub samples: Samples,
    ///No documentation.
    pub vae: Vae,
}
///**VAEEncodeAudio**
pub struct VaeEncodeAudio<Audio: crate::nodes::Audio, Vae: crate::nodes::Vae> {
    ///No documentation.
    pub audio: Audio,
    ///No documentation.
    pub vae: Vae,
}
