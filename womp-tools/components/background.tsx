import Image from 'next/image';
import background from '@/public/machariel.png';

export default function Background() {
    return <Image src={background} placeholder='blur' fill sizes='100vw' style={{ objectFit: 'cover', zIndex: -1 }} alt={'machariel-background'}/>
}