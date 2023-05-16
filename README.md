# resetti-manager
A binary manager for resetti.

# Installation
- Download the binary from the latest release.
- Export it into a file that has executable permissions for your user. If you don't know what that means, export it to your `Downloads` folder.
- Set the executable flag on the binary by performing this command in a terminal in the directory where you downloaded it. Here we take the example of `Downloads`.
```bash
cd ~/Downloads
chmod +x resetti-manager
```

# Usage
- The manager uses the `~/.cache/resetti-manager` directory to manage its package indices.
- You have two command line options that you can run:
  - Update
    - Updates the indices in `~/.cache/resetti-manager` without installing the binary to `/usr/bin`.
    ```bash
    ./resetti-manager -U
    # or
    ./resetti-manager --update
    ```
  - Upgrade
    - Updates the indices in `~/.cache/resetti-manager` and installs the binary to `/usr/bin`.
    ```bash
    ./resetti-manager -u
    # or
    ./resetti-manager --upgrade
    ```
# Thanks
- Woofdoggo for creating the [macro](https://github.com/woofdoggo/resetti).
