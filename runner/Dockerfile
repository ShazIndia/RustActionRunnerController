# Use the official runner image as the base image
FROM summerwind/actions-runner:latest

# Switch to root user to install dependencies
USER root

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install any additional dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    jq

# Revert back to the default user
USER runner

# Set the entrypoint
ENTRYPOINT ["/entrypoint.sh"]
