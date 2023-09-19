'use client';
export default function InputForm(){
    function handleSubmit(e: any) {
        console.log(e);
    }

    return (
        <>
        <form className="flex flex-col justify-center" method="post" onSubmit={handleSubmit}>
            <label className="p-2">
                Username:
                <input name="username" />
            </label>
            <label className="p-2">
                Password:
                <input type="password" name="password" />
            </label>
            <button type="submit">Login</button>
        </form>
        </>
    );
}