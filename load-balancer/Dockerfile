# Use the official Ubuntu base image
FROM ubuntu:latest

# Update package lists and install Python
RUN apt-get update && apt-get install -y python3

# Set the working directory in the container
WORKDIR /app

# Copy the Python script into the container
COPY load-balancer.py .

# Run the Python script when the container launches
CMD ["python3", "load-balancer.py"]
