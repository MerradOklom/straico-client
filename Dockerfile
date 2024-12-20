# Use Ubuntu as the base image
FROM ubuntu:22.04

# Set the working directory
WORKDIR /usr/local/bin

# Install required libraries including OpenSSL
RUN apt-get update && \
    apt-get install -y curl libssl3 && \
    apt-get clean

# Download the precompiled binary
RUN curl -L -o straico-proxy https://github.com/ricardokl/straico-client/releases/download/master/straico-proxy-linux

# Make the binary executable
RUN chmod +x ./straico-proxy

# Expose the port the app runs on
EXPOSE 8000

# Command to run the application, binding to 0.0.0.0 and port 8000
CMD ["./straico-proxy", "--host", "0.0.0.0", "--port", "8000"]
