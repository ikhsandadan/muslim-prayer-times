"use client";
import { usePathname } from 'next/navigation';
import FrontPage from './FrontPage';

export default function Home() {
  const pathname = usePathname();

  return (
    <>
    {pathname === '/' && <FrontPage />}
    </>
  )
};