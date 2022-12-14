use bevy::prelude::*;

pub mod ui {
    use super::*;
    pub mod menu {
        use super::*;

        /// 菜单UI主体
        #[derive(Component)]
        pub struct Main;

        /// 游戏标题
        #[derive(Component)]
        pub struct Title;

        /// 开始按钮
        #[derive(Component)]
        pub struct StartButton;

        /// 退出游戏按钮
        #[derive(Component)]
        pub struct ExitButton;
    }
}
