use radix_yew_progress::*;
use yew::prelude::*;

#[function_component]
pub fn ProgressDemo() -> Html {
    let progress_value = 65;

    html!{
        <div class="flex items-center">
            <label class="text-white text-[15px] leading-none pr-[15px]" for="loading-content">
                {"Loading..."}
            </label>
            <Progress
                id="loading-content"
                value="40"
                max="100"
                class="relative overflow-hidden bg-blackA6 rounded-full w-[300px] h-[25px] shadow-[0_2px_10px] shadow-blackA4"
            >
                <ProgressIndicator 
                    class="bg-white h-full w-full rounded-full transition-transform duration-[660ms] ease-[cubic-bezier(0.65,_0,_0.35,_1)]"
                />
            </Progress>
        </div>
    }
}