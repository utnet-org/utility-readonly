FROM gitpod/workspace-full

RUN git clone https://github.com/utnet-org/utility.git --depth 1 /home/gitpod/utility
RUN bash -cl "cd /home/gitpod/utility && cargo build && cargo test"
