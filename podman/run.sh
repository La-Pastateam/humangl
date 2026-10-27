#!/bin/bash

# Check if the Podman image exists locally, and build it automatically if it is missing
if ! podman image exists rust-vulkan-env; then
	echo "[INFO] Image rust-vulkan-env not found. Building it automatically..."
	podman build -t rust-vulkan-env -f podman/Containerfile .
fi

# Grant local display access
xhost +local: 2>/dev/null || true

# Initialize auth variable
XAUTH_ARGS=""

# Check if Xauthority exists on host and set auth flags
if [ -f "$HOME/.Xauthority" ]; then
  XAUTH_ARGS="-v $HOME/.Xauthority:/root/.Xauthority:ro -e XAUTHORITY=/root/.Xauthority"
fi

# Run the container with the following configuration:
#   - Run interactive container and delete on exit (--rm -it)
#   - Pass host GPU device (--device /dev/dri)
#   - Mount X11 socket read-only (-v /tmp/.X11-unix...)
#   - Apply X11 auth args if any ($XAUTH_ARGS)
#   - Forward display variable (-e DISPLAY=$DISPLAY)
#   - Force Winit to use X11 (-e WINIT_UNIX_BACKEND=x11)
#   - Mount current project folder (-v $(pwd):/app:z)
#   - Specify image name (rust-vulkan-env)
#   - Forward script arguments (e.g., cargo run)
podman run --rm -it \
  --device /dev/dri \
  -v /tmp/.X11-unix:/tmp/.X11-unix:ro \
  $XAUTH_ARGS \
  -e DISPLAY=$DISPLAY \
  -e WINIT_UNIX_BACKEND=x11 \
  -v $(pwd):/app:z \
  rust-vulkan-env \
  "$@"
