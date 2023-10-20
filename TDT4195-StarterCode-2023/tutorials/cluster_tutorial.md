# Visual Computing Server
This server is used for exercises in TDT4195, TDT4265, and TDT17.


## Information

By logging into snotra.idi.no you will be able to request GPU resources to speed up your deep learning training jobs.
The system is only accessible from an NTNU IP address, so you will have to use **VPN** to access it from elsewhere.

## How to:

**NOTE**: For all commands, replace the username **michaesl** with your own NTNU username.

Follow these steps for the initial setup

1. Download VScode (https://code.visualstudio.com/download)

2. Install "Remote - SSH" vscode extension (localy)

3. In the Remote Explorer extension add a new remote server (+)
```
    ssh -J michaesl@snotra.idi.ntnu.no michaesl@oppdal
```

4. Refresh the list of remotes and connect to oppdal

5. Install Python and Jupyter extension while logged into oppdal

The initial setup is now done, and you can run your python scripts and jupyter notebooks directly in vscode using the GPU resources at the server.

NOTE: Log out of the session when you are done working on the server (close the VScode window)


The next time you log into the server you only need to press connect to oppdal and you are good to go!


### Rules
Breaking the following rule can cause you to permanently loose access to the given compute resources.
- Using the cluster for any jobs which are not part of TDT4195, TDT4200, TDT4265, or TDT17 is not allowed.

### NTNU Home restrictions
Note that your home directory `/ntnu/home` is restricted in the amount of data you can save there.
Therefore, we recommend you to save models to `/work/snotra/michaesl` to prevent filling up the ntnu home directory.

### Uploading files
You can upload files with the following methods:
- Use git (inside or outside the docker container).
- "Drag and drop" with the VScode Explorer view.
- Synchronize files with tools such as: rsync, sshfs, or whatever floats your boat.
- NTNU-Home: the ~/ntnu-home folder will be a symlink to your NTNU home directory


**NOTE:** We do not have a backup of your files on the server, so make sure to backup your work now and then (git is a good tool for this). Also, at the end of the semester we will delete all your files.


### Notes
As the system allocates a full GPU per user, you will be able to utilize the full GPU, which in turn makes it important that you try to only use the server for GPU intensive workloads (i.e. training your neural networks).
Debugging your code should preferably be done locally, and once you are sure you want to train to completion, you should move to oppdal (remember to double check that you move your training from CPU to GPU when you do this).

### Allocation specifications

Each compute resource allocation will have the following specification:

- 4 CPU cores @2.10Ghz
- NVIDIA T4 with 16GB of VRAM
- 16GB of RAM
