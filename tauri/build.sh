pwd
ls -alh
ls -alh ..
# Remove fireside-chat .git
rm -fr fireside-chat/.git
# Clone Candle
cd .. && git clone https://github.com/huggingface/candle.git
rm -fr candle/.git
# CUDA
echo "Installed cuda version is: ${{steps.cuda-toolkit.outputs.cuda}}"
echo "Cuda install location: ${{steps.cuda-toolkit.outputs.CUDA_PATH}}"
nvcc -V
sudo ln -f -s /usr/local/cuda-12.3/targets/x86_64-linux/lib/stubs/libcuda.so /usr/local/cuda-12.3/targets/x86_64-linux/lib/stubs/libcuda.so.1
# APT
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf tar
sudo apt-get clean -y
# Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
rustup target add wasm32-unknown-unknown
# Download Trunk
cd ~/.cargo/bin
curl -L https://github.com/trunk-rs/trunk/releases/download/v0.18.8/trunk-x86_64-unknown-linux-gnu.tar.gz | tar xvz
