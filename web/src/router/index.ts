import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue';
import HelpView from '../views/HelpView.vue';
import TView from '../views/Template.vue';
import PredView from '../views/DemandPredition.vue';
import Setting from '../views/SettingView.vue';

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/help',
      name: 'help',
      component: HelpView
    },
    {
      path: '/pat/:share',
      name: 'pattern_share',
      component: PredView
    },
    {
      path: '/pred/:data',
      name: 'predition_data',
      component: PredView
    },
    {
      path: '/pred',
      name: 'predition',
      component: PredView
    },
    {
      path: '/plan',
      name: 'plan',
      component: TView
    },
    {
      path: '/setting',
      name: 'setting',
      component: Setting
    }
  ]
})

export default router
