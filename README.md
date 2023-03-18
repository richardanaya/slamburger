# 🍔 Slamburger [in-progress]

This repo is meant to be an extremely teachable codebase for how to do SLAM. It's also meant to provide a library in WebAssembly.

Features

- [x] Greyscale and blur image
- [x] Identify image feature keypoints with orientation
- [x] Calculate BREIF descriptors per keypoint
- [x] Identify similar keypoints using hamilton distance
- [x] Calculate 3D transform of movement between two images' keypoints using 8-point algorithm and RANSAC
- [ ] Test to verify things work and find bugs
- [ ] See realtime preview of it working
- [ ] Try to generate a 3D map ...

See the [demo](https://richardanaya.github.io/slamburger/index.html)

# What is SLAM?

SLAM, or Simultaneous Localization and Mapping, is a technique used by robots, drones, or other devices to create a map of their surroundings while simultaneously figuring out their own position within that map. To explain it in simpler terms, it's like giving a device the ability to "see" its environment and understand its location at the same time. This is particularly useful for devices that need to navigate through an unknown or changing environment.

The two main aspects of SLAM are identifying 3D transformations between images and creating 3D meshes of rooms. Let's break down these concepts further:

- **Identifying 3D transformations between images**: This is about understanding how one image is related to another in terms of position, orientation, and scale. Imagine you are in a room and take a picture from one corner, then walk to the other corner and take another picture. The SLAM system needs to figure out how these two images are related, even though they have different perspectives. It does this by finding common features or landmarks in both images and determining how they have shifted, rotated, or scaled relative to each other. This information helps the system understand the device's movement in the 3D space.

- **Creating 3D meshes of rooms**: A 3D mesh is a representation of a room's structure using interconnected points, lines, and faces. Think of it like a wireframe model of a room. By combining the information from multiple images taken from different perspectives, SLAM can create a 3D mesh that accurately represents the room's shape, size, and layout. The more images the system has, the more detailed and accurate the 3D mesh becomes.

So, in essence, SLAM enables a device to "see" its environment and understand its position within it by analyzing images, recognizing common features, and constructing a 3D mesh of the space. This technology is widely used in robotics, augmented reality, and autonomous vehicles to help them navigate and interact with their surroundings more effectively.

![download (1)](https://user-images.githubusercontent.com/294042/225192592-14ff5f43-fdea-4fe1-afa1-470e5eeb59fb.png)
