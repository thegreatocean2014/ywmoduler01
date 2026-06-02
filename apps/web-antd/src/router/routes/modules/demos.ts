import type { RouteRecordRaw } from 'vue-router';

import { $t } from '#/locales';

const routes: RouteRecordRaw[] = [
  {
    meta: {
      icon: 'ic:baseline-view-in-ar',
      keepAlive: true,
      order: 1000,
      title: $t('demos.title'),
    },
    name: 'Demos',
    path: '/demos',
    children: [
      {
        meta: {
          title: $t('demos.antd'),
        },
        name: 'AntDesignDemos',
        path: '/demos/ant-design',
        component: () => import('#/views/demos/antd/index.vue'),
      },

      {
        name: 'VxeTableExample',
        path: '/demos/vxe-table',
        meta: {
          icon: 'lucide:table',
          title: $t('demos.vxeTable.title'),
        },
        children: [
          {
            name: 'VxeTableBasicExample',
            path: '/demos/vxe-table/basic',
            component: () => import('#/views/demos/vxe-table/basic.vue'),
            meta: {
              title: $t('demos.vxeTable.basic'),
            },
          },
          {
            name: 'VxeTableRemoteExample',
            path: '/demos/vxe-table/remote',
            component: () => import('#/views/demos/vxe-table/remote.vue'),
            meta: {
              title: $t('demos.vxeTable.remote'),
            },
          },
          {
            name: 'VxeTableTreeExample',
            path: '/demos/vxe-table/tree',
            component: () => import('#/views/demos/vxe-table/tree.vue'),
            meta: {
              title: $t('demos.vxeTable.tree'),
            },
          },
          {
            name: 'VxeTableFixedExample',
            path: '/demos/vxe-table/fixed',
            component: () => import('#/views/demos/vxe-table/fixed.vue'),
            meta: {
              title: $t('demos.vxeTable.fixed'),
            },
          },
          {
            name: 'VxeTableCustomCellExample',
            path: '/demos/vxe-table/custom-cell',
            component: () =>
                import('#/views/demos/vxe-table/custom-cell.vue'),
            meta: {
              title: $t('demos.vxeTable.custom-cell'),
            },
          },
          {
            name: 'VxeTableFormExample',
            path: '/demos/vxe-table/form',
            component: () => import('#/views/demos/vxe-table/form.vue'),
            meta: {
              title: $t('demos.vxeTable.form'),
            },
          },
          {
            name: 'VxeTableEditCellExample',
            path: '/demos/vxe-table/edit-cell',
            component: () => import('#/views/demos/vxe-table/edit-cell.vue'),
            meta: {
              title: $t('demos.vxeTable.editCell'),
            },
          },
          {
            name: 'VxeTableEditRowExample',
            path: '/demos/vxe-table/edit-row',
            component: () => import('#/views/demos/vxe-table/edit-row.vue'),
            meta: {
              title: $t('demos.vxeTable.editRow'),
            },
          },
          {
            name: 'VxeTableVirtualExample',
            path: '/demos/vxe-table/virtual',
            component: () => import('#/views/demos/vxe-table/virtual.vue'),
            meta: {
              title: $t('demos.vxeTable.virtual'),
            },
          },
          {
            name: 'VxeTableViewedExample',
            path: '/demos/vxe-table/viewed',
            component: () => import('#/views/demos/vxe-table/viewed.vue'),
            meta: {
              title: $t('demos.vxeTable.viewed'),
            },
          },
        ],
      },
    ],
  },
];

export default routes;
