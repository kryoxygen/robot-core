#!/bin/bash

set -e

script=$(readlink -f "$0")
route=$(dirname "$script")

sudo apt update
# assume you have sourced ROS environment, same blow
sudo apt install -y libgflags-dev nlohmann-json3-dev \
ros-jazzy-image-transport  ros-jazzy-image-transport-plugins ros-jazzy-compressed-image-transport \
ros-jazzy-image-publisher ros-jazzy-camera-info-manager \
ros-jazzy-diagnostic-updater ros-jazzy-diagnostic-msgs ros-jazzy-statistics-msgs \
ros-jazzy-backward-ros libdw-dev libpcap-dev ros-jazzy-pcl-ros libcurlpp-dev