# Use an official Node.js runtime as a base image
FROM node:lts-bookworm-slim

# Set the working directory in the container
WORKDIR /app

# Copy package.json and package-lock.json (if you have one)
COPY package*.json ./

# Install application dependencies
RUN npm install

# Copy the application source code
COPY . .

# Set the entrypoint to execute your CLI script
ENTRYPOINT ["node", "cli.js"]

# You can optionally define a default command if your CLI takes arguments
# CMD ["--input", "some_file.txt", "--output", "compressed.dat"]