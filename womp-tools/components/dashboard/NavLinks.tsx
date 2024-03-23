'use client'

import Link from "next/link"
import { usePathname } from "next/navigation";
import clsx from 'clsx';
import { HomeIcon, UserIcon, CubeIcon, PresentationChartLineIcon } from '@heroicons/react/24/outline';

const links = [
    { name: 'Home', href: '/dashboard', icon: HomeIcon },
    { name: 'Characters', href: '/dashboard/characters', icon: UserIcon },
    { name: 'Assets', href: '/dashboard/assets', icon: CubeIcon },
    { name: 'Market', href: '/dashboard/market', icon: PresentationChartLineIcon },
]

export default function NavLinks() {
    const pathname = usePathname();

    return (
        <>
            {links.map((link) => {
                return (
                    <Link 
                        key={link.name} 
                        href={link.href} 
                        className={clsx('flex h-[48px] grow items-center justify-center gap-2 rounded-md bg-gray-800 p-3 text-sm font-medium hover:bg-gray-700 hover:text-blue-300 md:flex-none md:justify-start md:p-2 md:px-3', {
                            'bg-gray-700 text-blue-300': pathname === link.href
                        })}
                    >
                        <link.icon className="w-6" />
                        <p className="hidden md:block">{link.name}</p>
                    </Link>
                )
            })}
        </>
    )
}