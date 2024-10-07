# thumb_gallery

## Project Overview:

We are building a simple web server that allows users to upload and view images, with automatic thumbnail generation. The server will offer several key endpoints to manage and display images, with a tagging feature for easy search.

## Endpoints:

GET `/` - Displays thumbnails of all uploaded images, with a form to upload new images.<br/>
GET `/images` - Returns a JSON list of all uploaded images (including metadata).<br/>
POST `/upload` - Allows uploading a new image. Automatically generates a thumbnail. <br/>
GET `/image/{id}` - Displays a full-sized image by its unique ID.<br/>
GET `/thumb/{id}` - Displays the thumbnail of an image by its unique ID.<br/>
POST `/search` - Allows users to search for images by tags and returns matching images.
