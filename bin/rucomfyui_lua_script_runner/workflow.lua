-- Example workflow script for rucomfyui_mlua
-- This script generates an image using Stable Diffusion XL
--
-- The following globals are provided by the runner:
--   comfy: the rucomfyui module
--   client: a connected ComfyUI client
--   prompt: the text prompt to use (optional, defaults to "a cat sleeping on a red chair")

local prompt = prompt or "a cat sleeping on a red chair"

-- List available checkpoints and let the user choose
local checkpoints = client:get_models("checkpoints")
if #checkpoints == 0 then
    error("No checkpoints found on the server")
end

print("Available checkpoints:")
for i, name in ipairs(checkpoints) do
    print(string.format("  %d. %s", i, name))
end

io.write("\nSelect a checkpoint [1]: ")
io.stdout:flush()
local input = io.read()
local choice = tonumber(input) or 1
if choice < 1 or choice > #checkpoints then
    error(string.format("Invalid choice: %d (must be 1-%d)", choice, #checkpoints))
end
local checkpoint_name = checkpoints[choice]
print(string.format("Using: %s\n", checkpoint_name))

-- Get object info and create a graph
local object_info = client:get_object_info()
local g = comfy.graph(object_info)

-- Build the workflow
local c = g:CheckpointLoaderSimple(checkpoint_name)
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

local output_path = "output.png"

-- Queue the workflow and wait for results, observing streaming progress.
local result = client:execute(g, {
    on_event = function(event)
        if event.type == "progress" then
            print(string.format("  progress: %d/%d (node %s)", event.value, event.max, event.node or "?"))
        elseif event.type == "executing" then
            print(string.format("  executing: node %s", event.node or "done"))
        elseif event.type == "preview" then
            -- Overwrite the output file on each preview update
            local f = io.open(output_path, "wb")
            f:write(event.data)
            f:close()
            print(string.format("  preview saved: %s", output_path))
        elseif event.type == "executed" then
            print(string.format("  executed: node %d (%d images)", event.node, #event.images))
        end
    end,
})

-- Save the final image from the preview node to disk
for _, image in ipairs(result[preview].images) do
    local f = io.open(output_path, "wb")
    f:write(image)
    f:close()
    print(string.format("Saved: %s", output_path))
end

print("Done!")
