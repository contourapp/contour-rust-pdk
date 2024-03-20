ARG IMAGE
FROM mcr.microsoft.com/devcontainers/${IMAGE}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get -y install --no-install-recommends 
RUN apt-get -y install pkgconf