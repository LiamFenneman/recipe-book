# Recipe Book

Web application to manage recipes in the browser.
Integrates with the ChatGPT for automatically creating recipes.

# Docker Instructions

### Build Image
```
docker build --rm -t recipebook .
```

### Run Container
```
docker run --rm --name recipebook -d --network=host recipebook
```
