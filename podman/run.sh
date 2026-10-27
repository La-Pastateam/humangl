#!/bin/bash

# Check if the Podman image exists locally, and build it automatically if it is missing
if ! podman image exists rust-vulkan-env; then
    echo "[INFO] Image rust-vulkan-env not found. Building it automatically..."
    podman build -t rust-vulkan-env -f podman/Containerfile .
fi

# --- Automatic display detection (X11 or Wayland) ---
if [ "$XDG_SESSION_TYPE" = "wayland" ] || [ -n "$WAYLAND_DISPLAY" ]; then
    echo "[INFO] Graphical environment detected: Wayland"
    
    # Configuration for Wayland inside the container:
    # We set XDG_RUNTIME_DIR to /tmp and mount the host socket there so libwayland can find it.
    GUI_ARGS="-e WINIT_UNIX_BACKEND=wayland -e WAYLAND_DISPLAY=$WAYLAND_DISPLAY -e XDG_RUNTIME_DIR=/tmp"
    if [ -n "$XDG_RUNTIME_DIR" ] && [ -n "$WAYLAND_DISPLAY" ]; then
        GUI_ARGS="$GUI_ARGS -v $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:/tmp/$WAYLAND_DISPLAY:ro"
    fi
else
    echo "[INFO] Graphical environment detected: X11"
    
    # Grant local display access if xhost is available
    if command -v xhost &> /dev/null; then
        xhost +local: 2>/dev/null || true
    fi

    # Initialize auth variable
    XAUTH_ARGS=""

    # Check if Xauthority exists on host and set auth flags
    if [ -f "$HOME/.Xauthority" ]; then
      XAUTH_ARGS="-v $HOME/.Xauthority:/root/.Xauthority:ro -e XAUTHORITY=/root/.Xauthority"
    fi
    
    # Specific configuration for X11
    GUI_ARGS="-e WINIT_UNIX_BACKEND=x11 -e DISPLAY=$DISPLAY $XAUTH_ARGS -v /tmp/.X11-unix:/tmp/.X11-unix:ro"
fi

# Run the container with the following configuration:
#   - Run interactive container and delete on exit (--rm -it)
#   - Pass host GPU device (--device /dev/dri)
#   - Apply dynamic GUI arguments (X11 or Wayland based on host session)
#   - Mount current project folder (-v $(pwd):/app:z)
#   - Specify image name (rust-vulkan-env)
#   - Forward script arguments (e.g., cargo run)
podman run --rm -it \
  --device /dev/dri \
  $GUI_ARGS \
  -v $(pwd):/app:z \
  rust-vulkan-env \
  "$@"
