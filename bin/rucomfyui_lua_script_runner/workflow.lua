-- Example workflow script for rucomfyui_mlua
-- This script generates an image using Stable Diffusion XL
--
-- The following globals are provided by the runner:
--   comfy: the rucomfyui module
--   client: a connected ComfyUI client
--   prompt: the text prompt to use (optional, defaults to "a cat sleeping on a red chair")

local prompt = prompt or "a cat sleeping on a red chair"

-- Get object info and create a graph
local object_info = client:get_object_info()
local g = comfy.graph(object_info)

-- Build the workflow
local c = g:CheckpointLoaderSimple("sd_xl_base_1.0.safetensors")
local preview = g:PreviewImage(
    g:VAEDecode {
        vae = c.vae,
        samples = g:KSampler {
            model = c.model,
            seed = 0,
            steps = 20,
            cfg = 8.0,
            sampler_name = "euler",
            scheduler = "normal",
            positive = g:CLIPTextEncode { text = prompt, clip = c.clip },
            negative = g:CLIPTextEncode { text = "text, watermark, blurry", clip = c.clip },
            latent_image = g:EmptyLatentImage { width = 1024, height = 1024, batch_size = 1 },
            denoise = 1.0
        }
    }
)

-- Queue the workflow and wait for results
local result = client:easy_queue(g)

-- Return the images from the preview node
return result[preview].images
