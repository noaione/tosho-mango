import math
from io import BytesIO
from typing import Generator

from PIL import Image

__all__ = (
    "descramble_target",
    "bytes_to_image",
)


def calc_block_size(width: int, height: int, rectbox: int) -> tuple[int, int]:
    if width < rectbox or height < rectbox:
        return None
    n = 8
    width = math.floor(width / n) * n
    height = math.floor(height / n) * n
    return math.floor(width / rectbox), math.floor(height / rectbox)


def uint32(x: int) -> int:
    return x & 0xFFFFFFFF


def seed_generator(base_seed: int) -> Generator[int, None, None]:
    t = [uint32(base_seed)]
    while True:
        # Clamp to unsigned 32-bit integer
        t[0] = uint32(t[0] ^ uint32(t[0] << 13))
        t[0] = uint32(t[0] ^ uint32(t[0] >> 17))
        t[0] = uint32(t[0] ^ uint32(t[0] << 5))
        yield t


def generate_copy_targets(
    rectbox: int,
    base_seed: int,
) -> Generator[tuple[tuple[int, int], tuple[int, int]], None, None]:
    seed_gen = seed_generator(base_seed)

    seed_arrays = []
    for i in range(rectbox**2):
        idx_seed = next(seed_gen)
        seed_arrays.append([int(idx_seed), i])

    seed_arrays.sort(key=lambda x: x[0])
    index_only = [x[1] for x in seed_arrays]

    for idx, idx_seed in enumerate(index_only):
        yield (
            (idx_seed % rectbox, math.floor(idx_seed / rectbox)),
            (idx % rectbox, math.floor(idx / rectbox)),
        )


def descramble_target(im_source: Image.Image, block_div: int, scramble_seed: int):
    """Descramble image

    Parameters
    ----------
    im_source: :class:`PIL.Image.Image`
        Source/Scrambled image
    block_div: :class:`int`
        How much block that divide the images, usually at ``4`` right now.
    scramble_seed: :class:`int`
        The seed to be used to generate the scramble pattern.
        You can find it from the API response from :meth:`KMClient.get_web_chapter_viewer`

    Returns
    -------
    :class:`PIL.Image.Image`
        The descrambled image
    """
    block_w, block_h = calc_block_size(im_source.width, im_source.height, block_div)
    canvas = Image.new("RGB", (block_w * block_div, block_h * block_div))

    for (source_x, source_y), (dest_x, dest_y) in generate_copy_targets(block_div, scramble_seed):
        source_x *= block_w
        source_y *= block_h
        dest_x *= block_w
        dest_y *= block_h
        canvas.paste(
            im_source.crop((source_x, source_y, source_x + block_w, source_y + block_h)),
            (dest_x, dest_y, dest_x + block_w, dest_y + block_h),
        )

    return canvas


def bytes_to_image(bytes_data: bytes) -> Image.Image:
    """Convert bytes to :class:`PIL.Image.Image`

    Parameters
    ----------
    bytes_data: :class:`bytes`
        The bytes data of the image

    Returns
    -------
    :class:`PIL.Image.Image`
        The image
    """
    return Image.open(BytesIO(bytes_data))
