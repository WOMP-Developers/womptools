/** @type {import('next').NextConfig} */
const nextConfig = {
    images: {
        remotePatterns: [
            {
                protocol: 'https',
                hostname: 'images.evetech.net',
                port: '',
            }
        ]
    }
};

export default nextConfig;
