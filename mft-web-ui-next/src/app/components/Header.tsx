import Link from "next/link";

export default function Header() {


    return (
    <>    
        <nav className="bg-white border-gray-200 dark:bg-gray-900 dark:border-gray-700">
            <div className="flex justify-between">
                <div className="pl-10">
                    MFT
                </div>
                <div className="">
                    <Link className="p-8" href={"/feeling"}>Feeling</Link>
                    <Link className="p-8" href={"/food/ingredient"}>Ingredient</Link>
                    <Link className="p-8" href={"/food/meal"}>Meal</Link>
                    <Link className="p-8" href={"/user/login"}>Login</Link>
                    <Link className="p-8" href={"/user/register"}>Register</Link>
                </div>
            </div>
        </nav>
    </>
)
}