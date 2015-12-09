# turbine
Also known as Flare

This is a Rust project, so you will need to download and install Rust. Go to https://www.rust-lang.org/index.html and click on the Install button. It will provide you the installer that's relevant to your system.

There is a developer documentation that is included with this README called "DevDoc.pdf". Please refer to it for additional details.

Make sure to do the following things before you try to build or run Turbine:

1) Install Rust. You may need to install additional files to get Cargo (which is the package manager/build tool for Rust) working if you're on Windows.

2) Make sure you have installed dependencies for Rust. These are:

	- g++ 4.7 or clang++ 3.x
	
	- python 2.6 or later (but not 3.x)
	
	- GNU make 3.81 or later
	
	- curl
	
	- git
	
3) Run setup.sh located under build folder. This installs Postgres if you don't have it already.

Once the above steps are complete, you should be good to go!

To build Turbine, run "cargo build" in the root directory of Turbine.