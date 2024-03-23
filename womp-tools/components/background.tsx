import Image from 'next/image';
import background from '@/public/machariel.png';

export default function Background() {
    return (
        <div className={'absolute h-screen w-screen -z-1 pointer-events-none'}>
            <Image src={background} placeholder='blur' fill sizes='100vw' style={{ objectFit: 'cover', zIndex: -1 }} alt={'machariel-background'} />
        </div>
    )
}