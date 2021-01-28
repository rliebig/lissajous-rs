import glob
import webp
from PIL import Image
import re
fp_in = "./*.png"
fp_out = "out.gif"

def atoi(text):
    return int(text) if text.isdigit() else text

def natural_keys(text):
    return [ atoi(c) for c in re.split(r'(\d+)', text) ]


img, *imgs = [Image.open(f) for f in sorted(glob.glob(fp_in), key=natural_keys)]
# the crate i use to extract images works
# not very well, therefore I do some make up on
# all resulting images.

# unpredictably, this inverts the image. It's fine by me - but what the hell?
for value in imgs:
    pixels = value.load()
    for i in range(img.size[0]):
        for j in range(img.size[1]):
            if pixels[i,j] > (200, 200, 200):
                pixels[i,j] = (0,0,0)

webp.save_images(imgs, "anim.webp", lossless=False, fps=30)