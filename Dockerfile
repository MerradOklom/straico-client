# Use a minimal base image
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/local/bin

# Install curl to download the binary
RUN apt-get update && apt-get install -y curl && apt-get clean

# Download the precompiled binary
RUN curl -L -o straico-proxy https://github.com/ricardokl/straico-client/releases/download/master/straico-proxy-linux

# Make the binary executable
RUN chmod +x ./straico-proxy

# Expose the port the app runs on
EXPOSE 8000

# Command to run the application
CMD ["./straico-proxy"]
