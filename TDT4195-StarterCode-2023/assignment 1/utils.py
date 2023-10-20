import torch
import numpy as np
import matplotlib.pyplot as plt
import os
import subprocess
import time


def compute_loss_and_accuracy(dataloader, model, loss_function):
    """
    Computes the total loss and accuracy over the whole dataloader
    Args:
        dataloder: Test dataloader
        model: torch.nn.Module
        loss_function: The loss criterion, e.g: nn.CrossEntropyLoss()
    Returns:
        [loss_avg, accuracy]: both scalar.
    """
    model.eval()
    # Tracking variables
    loss_avg = 0
    total_correct = 0
    total_images = 0
    total_steps = 0
    with torch.no_grad():  # No need to compute gradient when testing
        for (X_batch, Y_batch) in dataloader:
            # Forward pass the images through our model
            X_batch, Y_batch = to_cuda([X_batch, Y_batch])
            output_probs = model(X_batch)
            # Compute loss
            loss = loss_function(output_probs, Y_batch)

            # Predicted class is the max index over the column dimension
            predictions = output_probs.argmax(dim=1).squeeze()
            Y_batch = Y_batch.squeeze()

            # Update tracking variables
            loss_avg += loss.cpu().item()
            total_steps += 1
            total_correct += (predictions == Y_batch).sum().item()
            total_images += predictions.shape[0]
    model.train()
    loss_avg = loss_avg / total_steps
    accuracy = total_correct / total_images
    return loss_avg, accuracy


def plot_loss(loss_dict, label):
    global_steps = list(loss_dict.keys())
    loss = list(loss_dict.values())
    plt.plot(global_steps, loss, label=label)


def read_im(filepath):
    im = plt.imread(filepath)
    if im.dtype == np.uint8:
        im = im.astype(float) / 255
    return im


def normalize(im):
    return (im - im.min()) / (im.max() - im.min())


def save_im(filepath, im, cmap=None):
    if im.min() < 0 or im.max() > 1:
        print(
            "Warning: The dynamic range of the image is",
            f"[{im.min()}, {im.max()}]",
            "normalizing to [-1, 1]")
        im = normalize(im)
    plt.imsave(filepath, im, cmap=cmap)


def to_cuda(elements):
    """
    Transfers all parameters/tensors to GPU memory (cuda) if there is a GPU available
    """
    if not torch.cuda.is_available():
        return elements
    if isinstance(elements, tuple) or isinstance(elements, list):
        return [x.cuda() for x in elements]
    return elements.cuda()


def assign_free_gpus(threshold_vram_usage=6000, max_gpus=1):
    """
    Assigns free gpus to the current process via the CUDA_AVAILABLE_DEVICES env variable
    This function should be called after all imports,
    in case you are setting CUDA_AVAILABLE_DEVICES elsewhere

    Borrowed from https://gist.github.com/afspies/7e211b83ca5a8902849b05ded9a10696

    Args:
        threshold_vram_usage (int, optional): A GPU is considered free if the vram usage is below the threshold
                                              Defaults to 6000 (MiB).
        max_gpus (int, optional): Max GPUs is the maximum number of gpus to assign.
                                  Defaults to 2.
        wait (bool, optional): Whether to wait until a GPU is free. Default False.
        sleep_time (int, optional): Sleep time (in seconds) to wait before checking GPUs, if wait=True. Default 10.
    """

    def _check():
        # Get the list of GPUs via nvidia-smi
        try:
            smi_query_result = subprocess.check_output(
                "nvidia-smi -q -d Memory | grep -A4 GPU", shell=True
            )
        except subprocess.CalledProcessError:
            return False
        # Extract the usage information
        gpu_info = smi_query_result.decode("utf-8").split("\n")
        gpu_info = list(filter(lambda info: "Used" in info, gpu_info))
        gpu_info = [
            int(x.split(":")[1].replace("MiB", "").strip()) for x in gpu_info
        ]  # Remove garbage
        # Keep gpus under threshold only
        free_gpus = [
            str(i) for i, mem in enumerate(gpu_info) if mem < threshold_vram_usage
        ]
        free_gpus = free_gpus[: min(max_gpus, len(free_gpus))]
        gpus_to_use = ",".join(free_gpus)
        return gpus_to_use

    gpus_to_use = _check()
    if gpus_to_use:
        os.environ["CUDA_VISIBLE_DEVICES"] = gpus_to_use
        print(f"Using GPU index: {gpus_to_use}")
        return
    
    print("No GPU available, using CPU")