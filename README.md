# Pictures Site - Photo Gallery on Internet Computer

A decentralized photo gallery application built on the Internet Computer blockchain. Users can upload, view, and manage their photos in a secure, decentralized environment.

## Features

- **Photo Upload**: Upload images directly to the Internet Computer
- **Photo Gallery**: View all uploaded photos in a responsive grid layout
- **Decentralized Storage**: Photos are stored on-chain using Internet Computer's asset storage
- **Secure**: Content Security Policy configured to prevent XSS attacks while allowing blob URLs for image display

## Architecture

### Backend (Rust)

- **Canister**: `pictures-site-backend`
- **Language**: Rust with ic-cdk
- **Functionality**: Handles photo storage, retrieval, and metadata management

### Frontend (TypeScript/React)

- **Canister**: `pictures-site-frontend`
- **Framework**: React with TypeScript
- **Build Tool**: Vite
- **Styling**: CSS with responsive design

## Getting Started

### Prerequisites

- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install) installed
- Node.js and npm

### Local Development

1. **Start the Internet Computer replica**:

   ```bash
   dfx start --clean --background
   ```

2. **Deploy the canisters**:

   ```bash
   dfx deploy
   ```

3. **Access the application**:
   - Frontend: `http://u6s2n-gx777-77774-qaaba-cai.localhost:4943/`
   - Backend Candid UI: `http://127.0.0.1:4943/?canisterId=uzt4z-lp777-77774-qaabq-cai&id=uxrrr-q7777-77774-qaaaq-cai`

### Development Workflow

- **Generate Candid interfaces** after backend changes:

  ```bash
  npm run generate
  ```

- **Frontend development server** (if needed):
  ```bash
  npm start
  ```

## Project Structure

```
src/
├── pictures-site-backend/          # Rust backend canister
│   ├── src/lib.rs                 # Main backend logic
│   ├── Cargo.toml                 # Rust dependencies
│   └── pictures-site-backend.did  # Candid interface
└── pictures-site-frontend/         # React frontend canister
    ├── src/
    │   ├── App.tsx                # Main React component
    │   ├── index.tsx              # Entry point
    │   └── *.css                  # Styling
    ├── public/
    │   ├── index.html             # HTML template
    │   └── .ic-assets.json5       # Asset configuration & CSP
    ├── package.json               # Node dependencies
    └── vite.config.ts             # Vite configuration
```

## Security Features

- **Content Security Policy**: Configured to prevent XSS attacks while allowing necessary resources
- **Blob URL Support**: CSP allows `blob:` URLs for displaying uploaded images
- **Frame Protection**: X-Frame-Options set to DENY to prevent clickjacking
- **HTTPS Enforcement**: Strict-Transport-Security header configured

## Technical Notes

### Content Security Policy Fix

The application includes a fix for displaying uploaded images by allowing `blob:` URLs in the CSP `img-src` directive. This resolves the issue where uploaded images would appear as broken links.

### Asset Configuration

The `.ic-assets.json5` file in the frontend public directory configures:

- Content Security Policy headers
- Permissions Policy for enhanced security
- Asset serving rules for the Internet Computer

## Contributing

1. Make your changes
2. Test locally with `dfx deploy`
3. Commit your changes with descriptive messages
4. Ensure all security policies are maintained

## Resources

- [Internet Computer Documentation](https://internetcomputer.org/docs/)
- [Rust CDK Documentation](https://docs.rs/ic-cdk)
- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)
