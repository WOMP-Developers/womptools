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
    },
    reactStrictMode: false,
};

export default nextConfig;
