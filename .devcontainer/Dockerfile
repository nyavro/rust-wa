FROM rust
RUN apt-get update && apt-get install -y --no-install-recommends \
    fzf \
    jq \
    less \
    tree \
    vim \    
    zsh \
    zsh-autosuggestions \
    zsh-syntax-highlighting \
    && chsh -s /usr/bin/zsh
ADD rc-files.txz /root/

