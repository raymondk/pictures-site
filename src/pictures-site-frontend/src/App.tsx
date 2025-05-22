import React, { useState, useEffect } from 'react';
import { pictures_site_backend } from '../../declarations/pictures-site-backend';
import './App.css';

interface ImageInfo {
  id: bigint;
  name: string;
  content_type: string;
}

interface ImageData {
  id: bigint;
  name: string;
  content_type: string;
  data: Uint8Array;
}

function App() {
  const [images, setImages] = useState<ImageInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [uploading, setUploading] = useState(false);
  const [uploadStatus, setUploadStatus] = useState('');

  useEffect(() => {
    loadImages();
  }, []);

  const loadImages = async () => {
    try {
      setLoading(true);
      const imageList = await pictures_site_backend.list_images();
      setImages(imageList);
    } catch (error) {
      console.error('Error loading images:', error);
      setUploadStatus('Error loading images');
    } finally {
      setLoading(false);
    }
  };

  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    if (!file.type.startsWith('image/')) {
      setUploadStatus('Please select an image file');
      return;
    }

    setUploading(true);
    setUploadStatus('Uploading...');

    try {
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);

      const imageId = await pictures_site_backend.upload_image(
        file.name,
        file.type,
        uint8Array
      );

      setUploadStatus(`Image uploaded successfully! ID: ${imageId}`);
      await loadImages(); // Refresh the image list

      // Clear the file input
      event.target.value = '';
    } catch (error) {
      console.error('Error uploading image:', error);
      setUploadStatus('Error uploading image');
    } finally {
      setUploading(false);
    }
  };

  const getImageUrl = (imageId: bigint) => {
    // Create a blob URL for the image
    return `/image/${imageId}`;
  };

  return (
    <div className="app">
      <div className="container">
        <h1>📸 Photo Gallery</h1>

        <div className="upload-section">
          <h2>Upload New Image</h2>
          <div className="upload-form">
            <input
              type="file"
              accept="image/*"
              onChange={handleFileUpload}
              disabled={uploading}
              className="file-input"
            />
            {uploadStatus && (
              <div className={`status ${uploadStatus.includes('Error') ? 'error' : 'success'}`}>
                {uploadStatus}
              </div>
            )}
          </div>
        </div>

        <div className="gallery-section">
          <h2>Gallery</h2>
          {loading ? (
            <div className="loading">Loading images...</div>
          ) : images.length === 0 ? (
            <div className="no-images">No images uploaded yet. Upload your first image above!</div>
          ) : (
            <div className="gallery">
              {images.map((image) => (
                <ImageCard key={image.id.toString()} image={image} />
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

interface ImageCardProps {
  image: ImageInfo;
}

function ImageCard({ image }: ImageCardProps) {
  const [imageData, setImageData] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadImageData();
  }, [image.id]);

  const loadImageData = async () => {
    try {
      const data = await pictures_site_backend.get_image(image.id);
      if (data && data.length > 0) {
        const imageDataObj = data[0] as ImageData;
        const blob = new Blob([imageDataObj.data], { type: imageDataObj.content_type });
        const url = URL.createObjectURL(blob);
        setImageData(url);
      }
    } catch (error) {
      console.error('Error loading image data:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="image-card">
      {loading ? (
        <div className="image-placeholder">Loading...</div>
      ) : imageData ? (
        <img src={imageData} alt={image.name} />
      ) : (
        <div className="image-placeholder">Failed to load</div>
      )}
      <div className="image-info">
        <div className="image-name">{image.name}</div>
      </div>
    </div>
  );
}

export default App;