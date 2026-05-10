/** @type {import('next').NextConfig} */
const nextConfig = {
  // Static export for Tauri production builds
  output: "export",
  // Custom output dir — must match tauri.conf.json frontendDist
  distDir: "../dist",
  // Images: unoptimized required for static export
  images: { unoptimized: true },
  // Required for static export with next-intl (no server)
  experimental: {},
};

export default nextConfig;
