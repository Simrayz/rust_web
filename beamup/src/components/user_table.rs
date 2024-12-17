use rust_html::{rhtml, Render, Template};

use super::button::ButtonColor;

pub struct UserTableComponent {}

impl Render for UserTableComponent {
    fn render(&self) -> Template {
        let header_cell_class = "text-left border border-slate-500 p-2";
        let cell_class = "border border-slashed border-slate-700 dark:border-slate-500 p-2";
        let row_class = "hover:bg-gray-200 hover:dark:bg-gray-800 cursor-pointer";

        rhtml! { r#"
        <div class="border border-slate-500 rounded-md overflow-hidden">
            <table class="table-fixed rounded-md w-full border-collapse text-gray-900 dark:text-gray-100">
                <thead>
                    <tr class="bg-gray-300 dark:bg-gray-700">
                        <th class="{header_cell_class}">Name</th>
                        <th class="{header_cell_class}">Age</th>
                        <th class="{header_cell_class}">Job</th>
                        <th class="{header_cell_class}">Action</th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="{row_class}">
                        <td class="{cell_class}">John Doe</td>
                        <td class="{cell_class}">30</td>
                        <td class="{cell_class}">Developer</td>
                        <td class="{cell_class}"><button class="{ButtonColor::Primary}">Edit</button></td>

                    </tr>
                    <tr class="{row_class}">
                        <td class="{cell_class}">Jane Doe</td>
                        <td class="{cell_class}">25</td>
                        <td class="{cell_class}">Designer</td>
                        <td class="{cell_class}"><button class="{ButtonColor::Primary}">Edit</button></td>
                    </tr>
                    <tr class="{row_class}">
                        <td class="{cell_class}">Bob Smith</td>
                        <td class="{cell_class}">40</td>
                        <td class="{cell_class}">Manager</td>
                        <td class="{cell_class}"><button class="{ButtonColor::Primary}">Edit</button></td>
                    </tr>
                    <tr class="{row_class}">
                        <td class="{cell_class}">Sarah Johnson</td>
                        <td class="{cell_class}">35 years</td>
                        <td class="{cell_class}">Sales</td>
                        <td class="{cell_class}"><button class="{ButtonColor::Primary}">Edit</button></td>
                    </tr>
                </tbody>
            </table>
        </div>
    "# }
    }
}
