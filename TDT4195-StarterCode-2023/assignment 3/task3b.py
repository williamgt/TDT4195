import utils
import skimage
import skimage.morphology
import numpy as np
import pathlib


def distance_transform(im: np.ndarray) -> np.ndarray:
    """
        A function that computes the distance to the closest boundary pixel.

        args:
            im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
        return:
            (np.ndarray) of shape (H, W). dtype=np.int32
    """
    # START YOUR CODE HERE ### (You can change anything inside this block)
    # You can also define other helper functions
    assert im.dtype == bool
    structuring_element = np.array([
        [1, 1, 1],
        [1, 1, 1],
        [1, 1, 1]
    ], dtype=bool)
    result = im.astype(np.int32)
    return result
    ### END YOUR CODE HERE ###


if __name__ == "__main__":
    # <-- This image is created in task3a
    im = utils.read_image("noisy-filtered.png",
                          image_folder=pathlib.Path("image_processed"))
    binary_image = (im != 0)
    distance = distance_transform(binary_image)

    assert im.shape == distance.shape, "Expected image shape ({}) to be same as resulting image shape ({})".format(
        im.shape, distance.shape)
    assert distance.dtype == np.int32, "Expected resulting image dtype to be np.int32. Was: {}".format(
        distance.dtype)

    distance = utils.to_uint8(distance)
    utils.save_im("noisy-distance.png", distance)
